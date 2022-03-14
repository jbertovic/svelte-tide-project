<script>
    export let menu = 1;
    export let loggedin;
</script>



<ul id ="menu">
	<li><a href="/" on:click|preventDefault={() => (menu = 1)}>Main</a></li>

	{#if !loggedin}
	<li><a href="/" on:click|preventDefault={() => (menu = 2)}>Login</a></li>
	{:else}
	<li><a href="/" on:click|preventDefault={() => (menu = 3)}>Secure</a></li>
	<li><a href="/" on:click|preventDefault={() => (menu = 4)}>Logout</a></li>
	{/if}
</ul>

<style>
	ul#menu li{
		display : inline;
		font-weight: 600;
		padding: 0.5em;
		border: 0.5em;
		border-style:solid;
		border-color: blue;
	}
	ul#menu {
		background: cyan;
		padding: 10px;

	}
</style>
