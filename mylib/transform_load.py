import sqlite3
import csv
import os

def load(dataset="data/drug-use-by-age.csv"):
    """Transforms and Loads data into the local SQLite3 database"""
    
    print(os.getcwd())
    
    with open(dataset, newline='', encoding='utf-8') as csvfile:
        reader = csv.DictReader(csvfile)
        
        conn = sqlite3.connect('DrugUseDB.db')
        c = conn.cursor()
        
        c.execute("DROP TABLE IF EXISTS DrugUse")
        
        c.execute("""
            CREATE TABLE DrugUse (
                age TEXT,
                n INTEGER,
                alcohol_use REAL,
                alcohol_frequency REAL,
                marijuana_use REAL,
                marijuana_frequency REAL,
                cocaine_use REAL,
                cocaine_frequency REAL,
                crack_use REAL,
                crack_frequency REAL,
                heroin_use REAL,
                heroin_frequency REAL,
                hallucinogen_use REAL,
                hallucinogen_frequency REAL,
                inhalant_use REAL,
                inhalant_frequency REAL,
                pain_releiver_use REAL,
                pain_releiver_frequency REAL,
                oxycontin_use REAL,
                oxycontin_frequency REAL,
                tranquilizer_use REAL,
                tranquilizer_frequency REAL,
                stimulant_use REAL,
                stimulant_frequency REAL,
                meth_use REAL,
                meth_frequency REAL,
                sedative_use REAL,
                sedative_frequency REAL
            )
        """)
        
        # 将数据插入表中
        data = []
        for row in reader:
            data.append((
                row['age'], 
                int(row['n']),
                float(row['alcohol_use']), 
                float(row['alcohol_frequency']), 
                float(row['marijuana_use']), 
                float(row['marijuana_frequency']), 
                float(row['cocaine_use']), 
                float(row['cocaine_frequency']) 
                if row['cocaine_frequency'] != '-' else None,
                float(row['crack_use']) 
                if row['crack_use'] != '-' else None, 
                float(row['crack_frequency']) 
                if row['crack_frequency'] != '-' else None, 
                float(row['heroin_use']) 
                if row['heroin_use'] != '-' else None, 
                float(row['heroin_frequency']) 
                if row['heroin_frequency'] != '-' else None, 
                float(row['hallucinogen_use']), 
                float(row['hallucinogen_frequency']), 
                float(row['inhalant_use']), 
                float(row['inhalant_frequency']) 
                if row['inhalant_frequency'] != '-' else None, 
                float(row['pain_releiver_use']), 
                float(row['pain_releiver_frequency']), 
                float(row['oxycontin_use']), 
                float(row['oxycontin_frequency']) 
                if row['oxycontin_frequency'] != '-' else None, 
                float(row['tranquilizer_use']), 
                float(row['tranquilizer_frequency']), 
                float(row['stimulant_use']), 
                float(row['stimulant_frequency']), 
                float(row['meth_use']) 
                if row['meth_use'] != '-' else None, 
                float(row['meth_frequency']) 
                if row['meth_frequency'] != '-' else None, 
                float(row['sedative_use']), 
                float(row['sedative_frequency'])
            ))
        
        c.executemany("""
            INSERT INTO DrugUse VALUES (
                      ?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)
        """, data)
        
        conn.commit()
        conn.close()
    
    return "DrugUseDB.db"