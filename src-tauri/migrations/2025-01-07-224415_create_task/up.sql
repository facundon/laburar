-- Your SQL goes here
CREATE TABLE task (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  name VARCHAR(255) NOT NULL,
  description TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE area (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  name VARCHAR(255) NOT NULL,
  description TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE assignment (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  task_id INTEGER NOT NULL,
  area_id INTEGER NOT NULL,
  difficulty INTEGER NOT NULL DEFAULT 1,
  frequency VARCHAR(255) NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (task_id) REFERENCES task(id),
  FOREIGN KEY (area_id) REFERENCES area(id),
  UNIQUE (task_id, area_id)
);

CREATE TABLE employee_assignment (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  employee_id INTEGER NOT NULL,
  assignment_id INTEGER NOT NULL,
  is_primary BOOLEAN DEFAULT FALSE,
  efficiency INTEGER NOT NULL DEFAULT 1,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  assigned_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (employee_id) REFERENCES employee(id),
  FOREIGN KEY (assignment_id) REFERENCES assignment(id),
  UNIQUE (employee_id, assignment_id)
);

INSERT INTO area (name) VALUES 
('Administración'),
('Recepción'),
('Pre Analítica'),
('Química'),
('Serología'),
('Endocrinología'),
('Bacteriología'),
('Hematología'),
('Derivaciones'),
('Parasitología'),
('Micología'),
('Limpieza'),
('Post Analítica'),
('Abastecimiento'),
('Comunicación'),
('Extracción');
