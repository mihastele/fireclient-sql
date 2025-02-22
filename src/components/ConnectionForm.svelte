<script lang="ts">
  interface ConnectionConfig {
    db_type: "mysql" | "postgres" | "mariadb";
    host: string;
    port: number;
    user: string;
    password?: string;
    database: string;
  }

  // Define the event type directly
  type ConnectEventHandler = (event: SubmitEvent) => void;

  export let config: ConnectionConfig;
  export let onConnect: ConnectEventHandler;
</script>

<form on:submit|preventDefault={onConnect}>
  <label>
    Database Type:
    <select bind:value={config.db_type}>
      <option value="mysql">MySQL/MariaDB</option>
      <option value="postgres">PostgreSQL</option>
    </select>
  </label>
  <label>Host: <input type="text" bind:value={config.host} /></label>
  <label>Port: <input type="number" bind:value={config.port} /></label>
  <label>User: <input type="text" bind:value={config.user} /></label>
  <label>Password: <input type="password" bind:value={config.password} /></label
  >
  <label>Database: <input type="text" bind:value={config.database} /></label>
  <button type="submit">Connect</button>
</form>
