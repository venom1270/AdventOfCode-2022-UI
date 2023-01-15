<script lang="ts">
	// https://schum123.github.io/svelte-loading-spinners/?ref=madewithsvelte.com
	import { Jumper } from 'svelte-loading-spinners';
	import { fade, fly } from 'svelte/transition';
	import { invoke } from '@tauri-apps/api/tauri'
	import { open } from '@tauri-apps/api/dialog';

	interface selectionType {
		id: string,
		text: string
	};
	let days = [...Array(25).keys()].map(n => ({id: 'day'+(n+1), text: 'Day ' + (n+1)}))
	let selectedDay: selectionType = days[0];

	$: buttonText = 'Run ' + selectedDay.text;
	
	let filePath: string;	
	let result: [number, number] | null = null;
	$: error = "";

	async function run() {
		result = null;
		filePath = "";
		let selected = await open({
			multiple: true,
			filters: [{
				name: 'Text file',
				extensions: ['txt'],
			}],
		});
		if (selected != null) {
			filePath = selected[0];
			await invoke(selectedDay.id, { filePath })
				.then((message) => {
					console.log(message)
					result = message as [number, number];
				})
				.catch((e) => {
					console.error(e)
					error = e;
				});
		} else {
			filePath = '';
		}
	}

	async function reset() {
		filePath = '';
		result = null;
		error = "";
	}

</script>

<div class="test">
	
	<div class="flexWrapper">
		<div class="wrapper">
			{#if result}
				<div in:fade={{ delay: 500 }}>
					<h1>Results</h1>
					<p>Part 1: <br><tt><strong>{result[0]}</strong></tt></p>
					<p>Part 2: <br><tt><strong>{@html result[1]}</strong></tt></p>
				</div>
			{:else if error != ""}
				<div in:fly="{{ x:-100, duration: 1000, delay: 500}}">
					<h1 style="color:red;">Error!</h1>
					<p>{error}</p>
				</div>
			{:else if filePath}
				<div out:fly|local="{{ x: 180, duration: 500 }}" in:fly="{{x: -200, duration: 500}}">
					<p>Running on file: {filePath}</p>
					<div class="flexCenter"><Jumper size="60" color="#FF3E00" unit="px" duration="1s" /></div>
				</div>
			
			{:else}
				<form>
					<label for="daySelect">Select day:</label>
					<select id="daySelect" bind:value={selectedDay}>
						{#each days as day}
							<option value={day}>
								{day.text}
							</option>
						{/each}
					</select>
					<div>
						<p>Select day and click the button button to import input file.</p>	
					</div>
				</form>
			{/if}
		</div>
	</div>

	{#if !result && !filePath}
		<button class="runButton" on:click="{run}">{buttonText}</button>
	{:else if filePath && result || error != ""}
		<button in:fade={{ delay: 1500, duration: 1000 }} class="resetButton" on:click="{reset}">Reset</button>
	{/if}

	
</div>


<style>
	select {
		width: 40%;
	}

	tt {
		font-size: large;
	}

	select {
		font-size: medium;
	}

	.resetButton {
		margin-top:50px;
	}

	button {
		background-color: #FF3E00;
		border-radius: 5px;
		border-color:transparent;
		color: white;
		font-weight: 550;
	}
	button:hover {
		background-color: #ff5820;
		border-radius: 5px;
		color: white;
	}
</style>
