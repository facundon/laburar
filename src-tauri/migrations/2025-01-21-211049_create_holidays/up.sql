-- Your SQL goes here
CREATE TABLE holiday (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  employee_id INTEGER NOT NULL,
  start_date DATE NOT NULL,
  end_date DATE NOT NULL,
  days_off INTEGER NOT NULL,
  notes TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (employee_id) REFERENCES employee(id) ON DELETE CASCADE
);

CREATE TABLE company_holiday (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  date DATE NOT NULL,
  description TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO company_holiday (date, description) VALUES
('2025-01-01', 'Año Nuevo'),
('2025-03-03', 'Lunes de Carnaval'),
('2025-03-04', 'Martes de Carnaval'),
('2025-03-24', 'Día Nacional de la Memoria por la Verdad y la Justicia'),
('2025-04-02', 'Día del Veterano y de los Caídos en la Guerra de Malvinas'),
('2025-04-18', 'Viernes Santo'),
('2025-05-01', 'Día del Trabajador'),
('2025-05-25', 'Día de la Revolución de Mayo'),
('2025-06-16', 'Día de Martín Miguel de Güemes'),
('2025-06-20', 'Día de la Bandera'),
('2025-07-09', 'Día de la Independencia'),
('2025-08-18', 'Día del Libertador José de San Martín'),
('2025-10-13', 'Día del Respeto a la Diversidad Cultural'),
('2025-11-24', 'Día de la Soberanía Nacional'),
('2025-12-08', 'Día de la Inmaculada Concepción'),
('2025-12-25', 'Navidad');
