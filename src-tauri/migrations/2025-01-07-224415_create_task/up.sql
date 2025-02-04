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
  FOREIGN KEY (task_id) REFERENCES task(id) ON DELETE CASCADE,
  FOREIGN KEY (area_id) REFERENCES area(id) ON DELETE CASCADE,
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
  FOREIGN KEY (employee_id) REFERENCES employee(id) ON DELETE CASCADE,
  FOREIGN KEY (assignment_id) REFERENCES assignment(id) ON DELETE CASCADE,
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

INSERT INTO task (name) VALUES
('Ingreso de pacientes'),
('Autorización de ordenes '),
('Extracción de sangre'),
('Control de coagulación '),
('Etiquetado de muestras'),
('Centrifugación de muestras'),
('Separación de sueros en areas'),
('Separacion de sueros en envíos'),
('Técnica de latex para AR'),
('Técnica de latex para VDRL'),
('Técnica de latex para ASO'),
('Dilución y lectura de ionogramas'),
('Manejo equipo INCA'),
('Técnica HAI toxoplasmosis'),
('Técnica HAI chagas');
