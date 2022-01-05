<script>
 import * as backend from "./turbocharger_generated";

 let store = new backend.Store();

 let counter = 0;

 setInterval(() => {
  store.set(counter++);
 }, 1000);
</script>

Hello!<br />

Store -> {$store}
