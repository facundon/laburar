-- Your SQL goes here
CREATE TABLE absence (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  employee_id INTEGER NOT NULL,
  is_justified BOOLEAN NOT NULL DEFAULT FALSE,
  will_return BOOLEAN NOT NULL DEFAULT FALSE,
  hours INTEGER NOT NULL,
  description TEXT,
  absence_type TEXT NOT NULL,
  absence_date DATE NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (employee_id) REFERENCES employee(id)
);

CREATE TABLE absence_return (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  absence_id INTEGER NOT NULL,
  returned_hours INTEGER NOT NULL,
  notes TEXT,
  return_date DATE NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (absence_id) REFERENCES absence(id)
);
