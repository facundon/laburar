<script lang="ts">
  import { invoke } from "../models/invoke"
  import { Employee } from "../models/employee"

  let name = $state("")
  let employeeGreetingMessage = $state("")

  async function addEmployee(event: Event) {
    event.preventDefault()
    let newEmployee = new Employee({
      firstName: name,
      lastName: "Doe",
      phone: "1234567890",
      address: "1234 Elm St",
      startDate: new Date(),
      createdAt: new Date(),
      id: 0,
    })
    try {
      newEmployee = await invoke(
        "create_employee_command",
        newEmployee.toCreateDTO(),
        Employee.fromDTO
      )
      employeeGreetingMessage = `Hello, ${newEmployee.name} [id: ${newEmployee.id}]!`
    } catch (error) {
      console.error("Failed to create employee:", error)
      employeeGreetingMessage = `Failed to create employee. Please try again. ${error}`
    }
  }
</script>

<main class="container">
  <h1>Welcome to Tauri + Svelte</h1>

  <p>Agregar Empleado</p>

  <form class="row" onsubmit={addEmployee}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>
  <p>{employeeGreetingMessage}</p>
</main>

<style>
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  .container {
    margin: 0;
    padding-top: 10vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }

  .row {
    display: flex;
    justify-content: center;
  }

  h1 {
    text-align: center;
  }

  input,
  button {
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #0f0f0f;
    background-color: #ffffff;
    transition: border-color 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  }

  button {
    cursor: pointer;
  }

  button:hover {
    border-color: #396cd8;
  }
  button:active {
    border-color: #396cd8;
    background-color: #e8e8e8;
  }

  input,
  button {
    outline: none;
  }

  #greet-input {
    margin-right: 5px;
  }
</style>
