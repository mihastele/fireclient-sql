<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import ConnectionForm from "../components/ConnectionForm.svelte";
  import TableView from "../components/TableView.svelte";

  interface ConnectionConfig {
    db_type: "mysql" | "postgres" | "mariadb";
    host: string;
    port: number;
    user: string;
    password?: string; // Make password optional
    database: string;
  }
  interface QueryResult {
    columns: string[];
    rows: { 0: string[] }[];
  }
  let connectionConfig: ConnectionConfig = {
    db_type: "mysql",
    host: "localhost",
    port: 3306,
    user: "root",
    password: "",
    database: "your_database", // Replace with your database
  };

  let connectionStatus: string = "";
  let queryResult: QueryResult | null = null;
  let query: string = "SELECT * FROM your_table"; // Replace with your table

  async function connect() {
    try {
      connectionStatus = await invoke("connect_to_db", {
        config: connectionConfig,
      });
      console.log(connectionStatus);
    } catch (error: any) {
      connectionStatus = `Error: ${error}`;
      console.error(error);
    }
  }

  async function runQuery() {
    try {
      queryResult = await invoke("execute_query", {
        config: connectionConfig,
        query,
      });
      console.log(queryResult);
    } catch (error: any) {
      queryResult = null;
      connectionStatus = `Query Error: ${error}`; // Show query errors
      console.error(error);
    }
  }
</script>

<main class="container mx-auto p-4 w-100">
  <ConnectionForm config={connectionConfig} onConnect={connect} />
  <p class="mt-2 text-gray-600">{connectionStatus}</p>
  <div class="mt-4">
    <label class="block text-sm font-medium text-gray-700"> SQL Query: </label>
    <textarea
      bind:value={query}
      class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring focus:ring-indigo-200 sm:text-sm"
      rows="5"
    ></textarea>
  </div>

  <button
    on:click={runQuery}
    disabled={!connectionStatus.startsWith("Successfully")}
    class="mt-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded disabled:bg-gray-400"
  >
    Run Query
  </button>

  {#if queryResult}
    <TableView data={queryResult} />
  {/if}
</main>
