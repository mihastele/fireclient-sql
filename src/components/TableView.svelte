<script lang="ts">
  interface QueryResult {
    columns: string[];
    rows: { 0: string[] }[]; // Array of objects, each with a '0' property holding the row data
  }

  export let data: QueryResult;
</script>

{#if data && data.columns.length > 0}
  <table>
    <thead>
      <tr>
        {#each data.columns as column}
          <th>{column}</th>
        {/each}
      </tr>
    </thead>
    <tbody>
      {#each data.rows as row}
        <tr>
          {#each row[0] as value}
            <td>{value}</td>
          {/each}
        </tr>
      {/each}
    </tbody>
  </table>
{:else}
  <p>No data to display or query returned no results.</p>
{/if}

<style>
  /* Basic table styling - customize as needed */
  table {
    width: 100%;
    border-collapse: collapse;
    margin-top: 1em;
  }

  th,
  td {
    border: 1px solid #ddd;
    padding: 8px;
    text-align: left;
  }

  th {
    background-color: #f2f2f2;
  }
</style>
