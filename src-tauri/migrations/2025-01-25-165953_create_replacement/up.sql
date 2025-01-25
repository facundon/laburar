-- Your SQL goes here
CREATE TABLE replacement (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  original_employee_id INTEGER NOT NULL,
  replacement_employee_id INTEGER NOT NULL,
  replacement_start_date DATE NOT NULL,
  replacement_end_date DATE NOT NULL,
  employee_assignment_id INTEGER NOT NULL,
  notes TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (original_employee_id) REFERENCES employee(id),
  FOREIGN KEY (replacement_employee_id) REFERENCES employee(id),
  FOREIGN KEY (employee_assignment_id) REFERENCES employee_assignment(id)
);
