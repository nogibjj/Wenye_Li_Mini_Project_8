import sqlite3
import logging

logging.basicConfig(
    level=logging.INFO,
    format='**%(asctime)s** - %(levelname)s - %(message)s',
    handlers=[
        logging.FileHandler("db_operations.md"),
        logging.StreamHandler()
    ]
)

def insert_row(query_data):
    """
    Insert a row into the DrugUse table.
    query_data: A tuple containing the values for the row to insert.
    """
    conn = sqlite3.connect("DrugUseDB.db")
    cursor = conn.cursor()
    
    query = """
        INSERT INTO DrugUse (
            age, n, alcohol_use, alcohol_frequency, marijuana_use, 
            marijuana_frequency, cocaine_use, cocaine_frequency, 
            crack_use, crack_frequency, heroin_use, heroin_frequency,
            hallucinogen_use, hallucinogen_frequency, inhalant_use, 
            inhalant_frequency, pain_releiver_use, pain_releiver_frequency, 
            oxycontin_use, oxycontin_frequency, tranquilizer_use, 
            tranquilizer_frequency, stimulant_use, stimulant_frequency, 
            meth_use, meth_frequency, sedative_use, sedative_frequency) 
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
    """
    
    # Execute the query with the provided data
    cursor.execute(query, query_data)
    conn.commit()
    
    logging.info("Executed INSERT query:\n```sql\n%s\n```", query)
    logging.info("Inserted a new row into the DrugUse table with data: %s", query_data)
    
    conn.close()
    return "Success"

def select_rows():
    """Select and print all rows from the DrugUse table."""
    conn = sqlite3.connect("DrugUseDB.db")
    cursor = conn.cursor()
    
    query = "SELECT * FROM DrugUse"
    cursor.execute(query)
    rows = cursor.fetchall()
    
    logging.info("Executed SELECT query:\n```sql\n%s\n```", query)
    logging.info("Selected rows:\n%s", rows)
    
    for row in rows:
        print(row)
    
    conn.close()
    return "Success"

def update_row(record_id, updates):
    """
    Update a specific row in the DrugUse table.
    record_id: The ID of the record to update.
    updates: A dictionary containing the columns to update and their new values.
    """
    conn = sqlite3.connect("DrugUseDB.db")
    cursor = conn.cursor()
    
    # Dynamically build the SQL update query based on the provided updates
    set_clause = ", ".join([f"{col} = ?" for col in updates.keys()])
    query = f"UPDATE DrugUse SET {set_clause} WHERE id = ?"
    
    # Execute the query with the new values followed by the record ID
    cursor.execute(query, (*updates.values(), record_id))
    conn.commit()
    
    logging.info("Executed UPDATE query:\n```sql\n%s\n```", query)
    logging.info("Updated the row with id = %s and new values: %s", record_id, updates)
    
    conn.close()
    return "Success"

def delete_row(record_id):
    """
    Delete a specific row from the DrugUse table.
    record_id: The ID of the record to delete.
    """
    conn = sqlite3.connect("DrugUseDB.db")
    cursor = conn.cursor()
    
    query = "DELETE FROM DrugUse WHERE id = ?"
    cursor.execute(query, (record_id,))
    conn.commit()
    
    logging.info("Executed DELETE query:\n```sql\n%s\n```", query)
    logging.info("Deleted the row with id = %s", record_id)
    
    conn.close()
    return "Success"