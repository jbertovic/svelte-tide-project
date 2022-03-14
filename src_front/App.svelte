<script>
	import {onMount} from 'svelte';
	import {user} from './store.js';
	import Navbar from "./component/Navbar.svelte";
	import Login from "./pages/Login.svelte";
	import Logout from "./pages/Logout.svelte";
	import Secure from "./pages/Secure.svelte";

	let menu;
    $: loggedin = $user !== '';

	// check if logged in
	onMount( async() => {
        const res = await fetch('/auth/session',{credentials: 'same-origin'});
        let sessionResponse = await res.json();
        if (sessionResponse.user_id !== '') {
            user.set(sessionResponse.user_id);
			loggedin = true;
		}else 
		{
			user.set('');
			loggedin = false;
		}
    });

</script>

<!-- MENNU BAR ON TOP -->
<Navbar bind:menu={menu} {loggedin} />

<!-- PAGE LOADING -->

{#if menu === 1}
	{#if !loggedin}
		<h4>Requires Login</h4>
	{:else}
		<h4>Logged In as {$user}</h4>
	{/if}
{:else if menu === 2}
	<Login />
{:else if menu === 3}
	<Secure/>
{:else if menu === 4}
	<Logout/>
{:else}
<h1>
	Page Not Found or Completed Yet
</h1>
{/if}

