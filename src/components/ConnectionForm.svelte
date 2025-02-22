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

<form
  on:submit|preventDefault={onConnect}
  class="bg-gray-50 py-12 px-4 sm:px-6 lg:px-8"
>
  <div class="max-w-md w-full mx-auto space-y-8">
    <div class="text-center">
      <h2
        class="text-3xl font-bold text-gray-900 bg-gradient-to-r from-blue-600 to-green-600 bg-clip-text text-transparent"
      >
        Database Connection
      </h2>
    </div>

    <div class="bg-white rounded-xl shadow-lg p-8 space-y-6">
      <div class="space-y-6">
        <!-- Database Type -->
        <div class="space-y-2">
          <label for="db_type" class="block text-sm font-medium text-gray-700">
            Database Type:
          </label>
          <select
            id="db_type"
            bind:value={config.db_type}
            class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition duration-200"
          >
            <option value="mysql">MySQL/MariaDB</option>
            <option value="postgres">PostgreSQL</option>
          </select>
        </div>

        <!-- Host/Port Row -->
        <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
          <div class="space-y-2">
            <label for="host" class="block text-sm font-medium text-gray-700">
              Host:
            </label>
            <input
              type="text"
              id="host"
              bind:value={config.host}
              class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition duration-200"
            />
          </div>

          <div class="space-y-2">
            <label for="port" class="block text-sm font-medium text-gray-700">
              Port:
            </label>
            <input
              type="number"
              id="port"
              bind:value={config.port}
              class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition duration-200"
            />
          </div>
        </div>

        <!-- User/Password Row -->
        <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
          <div class="space-y-2">
            <label for="user" class="block text-sm font-medium text-gray-700">
              User:
            </label>
            <input
              type="text"
              id="user"
              bind:value={config.user}
              class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition duration-200"
            />
          </div>

          <div class="space-y-2">
            <label
              for="password"
              class="block text-sm font-medium text-gray-700"
            >
              Password:
            </label>
            <input
              type="password"
              id="password"
              bind:value={config.password}
              class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition duration-200"
            />
          </div>
        </div>

        <!-- Database -->
        <div class="space-y-2">
          <label for="database" class="block text-sm font-medium text-gray-700">
            Database:
          </label>
          <input
            type="text"
            id="database"
            bind:value={config.database}
            class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition duration-200"
          />
        </div>
      </div>

      <!-- Connect Button -->
      <button
        type="submit"
        class="w-full bg-gradient-to-r from-blue-600 to-blue-500 hover:from-blue-700 hover:to-blue-600 text-white font-semibold py-3 px-4 rounded-lg transition-all duration-200 transform hover:scale-[1.01]"
      >
        Connect
      </button>
    </div>
  </div>
</form>
