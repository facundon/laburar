// Función asignar_tareas(empleados):

//         Para cada candidato (empleado disponible):
//             - Calcular la carga de tareas actuales del candidato:
//                 Sumar la dificultad de todas las tareas asignadas al candidato.

//             - Calcular el puntaje base del candidato para la tarea:
//                 Puntaje = (Eficiencia del candidato - Dificultad de la tarea) / (1 + Carga de tareas)

//             - Ajustar puntaje por vacaciones:
//                 Calcular el tiempo disponible del empleado antes de sus vacaciones.
//                 Calcular la duración de la tarea (diferencia entre fecha inicio y fecha fin).

//                 Si el tiempo disponible es suficiente para completar la tarea:
//                     - No aplicar penalización por vacaciones.
//                 Si el tiempo disponible no es suficiente:
//                     - Aplicar una penalización proporcional al tiempo que falta:
//                       Penalización = (Duración de la tarea - Tiempo disponible) / Duración de la tarea
//                       Restar esta penalización al puntaje.

//                 Si el empleado tiene menos de 5 días para irse de vacaciones:
//                     - Añadir un pequeño ajuste adicional (por ejemplo, Penalización = 0.3) para reflejar la dificultad logística de asignarle la tarea.
//                     - Restar este ajuste del puntaje.

//         Ordenar los candidatos por puntaje (de mayor a menor).

use diesel::SqliteConnection;

use crate::db::models::employee::list_competent_employees_for_assignment;

struct EmployeeSimple {
    id: i32,
    first_name: String,
    last_name: String,
}

pub enum SuggestionResult<'a> {
    Employees(Vec<EmployeeSimple>),
    Message(&'a str),
}

pub fn sugest_employees_for_assignation(
    conn: &mut SqliteConnection,
    assignment_id: i32,
) -> SuggestionResult {
    let employees = list_competent_employees_for_assignment(conn, assignment_id).unwrap();
    if employees.len() == 0 {
        return SuggestionResult::Message("No employees available for this task.");
    }
    println!("{:#?}", employees);
    return SuggestionResult::Message("Done");
    // for employee in employees {}
}
