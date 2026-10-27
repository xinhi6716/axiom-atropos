"""Long-run memory: verified units distilled into a local searchable store, plus a short digest per project."""
import contextlib
import hashlib
import json
import re
import sqlite3
import time
from pathlib import Path

from procs import BridgeError, require_slug

SCHEMA = """
CREATE TABLE IF NOT EXISTS entries (
    id INTEGER PRIMARY KEY, project TEXT NOT NULL, unit TEXT NOT NULL, title TEXT NOT NULL, summary TEXT NOT NULL,
    body TEXT NOT NULL, symbols TEXT NOT NULL, wish TEXT NOT NULL, files TEXT NOT NULL, project_dir TEXT NOT NULL, created REAL NOT NULL);
CREATE VIRTUAL TABLE IF NOT EXISTS entries_fts USING fts5(
    title, summary, body, symbols, wish, content='entries', content_rowid='id', tokenize='trigram');
CREATE TRIGGER IF NOT EXISTS entries_ai AFTER INSERT ON entries BEGIN
    INSERT INTO entries_fts(rowid, title, summary, body, symbols, wish)
    VALUES (new.id, new.title, new.summary, new.body, new.symbols, new.wish);
END;
"""
# The wish is indexed verbatim so entries written in English stay findable in the language the user wishes in.
FIELDS = ('title', 'summary', 'body', 'symbols', 'wish')
CJK = re.compile(r'[぀-ヿ㐀-鿿豈-﫿]+')
WORD = re.compile(r'[A-Za-z0-9_./-]{2,}')


def query_terms(query):
    """Trigram matching needs no word breaks, but Chinese words are often two characters, so CJK runs become bigrams."""
    terms = [word.lower() for word in WORD.findall(query)]
    for run in CJK.findall(query):
        terms += [run] if len(run) == 1 else [run[i:i + 2] for i in range(len(run) - 1)]
    return list(dict.fromkeys(terms))


def _hash(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


class Memory:
    def __init__(self, home):
        self.root = Path(home) / 'memory'
        self.root.mkdir(parents=True, exist_ok=True)
        self.db_path = self.root / 'memory.db'
        with self._connect() as db:
            db.executescript(SCHEMA)

    @contextlib.contextmanager
    def _connect(self):
        db = sqlite3.connect(self.db_path)
        db.row_factory = sqlite3.Row
        try:
            with db:
                yield db
        finally:
            db.close()

    def record(self, project, unit, title, summary, body, symbols, files, project_dir, wish):
        require_slug('project', project)
        for name, value in (('unit', unit), ('title', title), ('summary', summary), ('body', body), ('wish', wish)):
            if not isinstance(value, str) or not value.strip():
                raise BridgeError('invalid_argument', f'{name} is required')
        root = Path(project_dir or '')
        if not root.is_absolute() or not root.is_dir():
            raise BridgeError('invalid_argument', f'project_dir must be an existing absolute directory: {project_dir!r}')
        hashed = []
        for rel in files or []:
            path = root / rel
            if not path.is_file():
                raise BridgeError('invalid_argument', f'file not found in project_dir: {rel}')
            hashed.append({'path': rel, 'sha256': _hash(path)})
        with self._connect() as db:
            cursor = db.execute(
                'INSERT INTO entries (project, unit, title, summary, body, symbols, wish, files, project_dir, created) VALUES (?,?,?,?,?,?,?,?,?,?)',
                (project, unit, title.strip(), summary.strip(), body.strip(), ' '.join(symbols or []), wish.strip(), json.dumps(hashed),
                 str(root), time.time()))
        return {'status': 'completed', 'id': cursor.lastrowid, 'files': hashed, 'error': None}

    def search(self, query, project=None, limit=5):
        terms = query_terms(query or '')
        if not terms:
            raise BridgeError('invalid_argument', 'query has no searchable terms')
        hits = ' + '.join('(' + ' OR '.join(f'f.{field} LIKE ?' for field in FIELDS) + ')' for _ in terms)
        params = [f'%{term}%' for term in terms for _ in FIELDS]
        where = 'WHERE matched > 0' + (' AND project = ?' if project else '')
        sql = (f'SELECT * FROM (SELECT e.*, {hits} AS matched FROM entries e JOIN entries_fts f ON f.rowid = e.id) '
               f'{where} ORDER BY matched DESC, created DESC LIMIT ?')
        with self._connect() as db:
            rows = db.execute(sql, params + ([project] if project else []) + [int(limit)]).fetchall()
        return {'status': 'completed', 'terms': terms, 'error': None, 'entries': [
            {'id': row['id'], 'project': row['project'], 'unit': row['unit'], 'title': row['title'], 'summary': row['summary'],
             'matched_terms': row['matched'], 'stale': self._stale(row)} for row in rows]}

    def get(self, entry_id):
        with self._connect() as db:
            row = db.execute('SELECT * FROM entries WHERE id = ?', (int(entry_id),)).fetchone()
        if row is None:
            raise BridgeError('not_found', f'no memory entry {entry_id}')
        entry = {key: row[key] for key in row.keys()}
        entry['files'] = json.loads(entry['files'])
        entry['symbols'] = entry['symbols'].split()
        entry['stale'] = self._stale(row)
        return {'status': 'completed', 'entry': entry, 'error': None}

    def digest(self, project, add=None):
        require_slug('project', project)
        path = self.root / f'{project}.md'
        if add:
            line = ' '.join(add.split())
            with path.open('a', encoding='utf-8', newline='\n') as handle:
                handle.write(f'- {time.strftime("%Y-%m-%d")} {line}\n')
        return {'status': 'completed', 'digest': path.read_text(encoding='utf-8') if path.exists() else '', 'error': None}

    @staticmethod
    def _stale(row):
        """True when a recorded file changed or vanished; None when the project directory is gone, so it cannot be checked."""
        root = Path(row['project_dir'])
        if not root.is_dir():
            return None
        for item in json.loads(row['files']):
            path = root / item['path']
            if not path.is_file() or _hash(path) != item['sha256']:
                return True
        return False
