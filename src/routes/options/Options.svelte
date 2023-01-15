<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri'

	let backendDelay = true;

	async function updateState() {
		console.log(backendDelay);
		await invoke("update_backend_delay", { delay: backendDelay })
			.then((message) => {
				console.log(message)
				//result = message as [number, number];
			})
			.catch((e) => {
				console.error(e)
				//error = e;
			});
	}

	async function getState() {
		await invoke("get_backend_delay", {  })
			.then((message) => {
				console.log(message)
				backendDelay = message as boolean;
			})
			.catch((e) => {
				console.error(e)
				//error = e;
			});
	}
	getState();


</script>

<div class="test">
	
	<div class="flexWrapper">
		<div class="wrapper">
			<label>
				<input type=checkbox bind:checked={backendDelay} on:click={updateState}>
				Slight backend delay for a smoother experience
			</label>
		</div>
	</div>

	
</div>


<style>

	input[type=checkbox] {
		accent-color: #ff5820;
		transform: scale(1.5);
		color: white;
	}
	
</style>
