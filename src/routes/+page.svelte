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
    } catch (error: any) {
      queryResult = null;
      connectionStatus = `Query Error: ${error}`; // Show query errors
      console.error(error);
    }
  }
</script>

<main>
  <ConnectionForm config={connectionConfig} onConnect={connect} />
  <p>{connectionStatus}</p>

  <label>
    SQL Query:
    <textarea bind:value={query}></textarea>
  </label>
  <button
    on:click={runQuery}
    disabled={!connectionStatus.startsWith("Successfully")}>Run Query</button
  >

  {#if queryResult}
    <TableView data={queryResult} />
  {/if}
</main>
