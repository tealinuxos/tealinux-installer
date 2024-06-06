<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';

	let json;
	let showModelName = false;

	async function handleGetJSON() {
		invoke('get_read_json').then((response) => {
			json = JSON.parse(response);
			console.log(json);
			showModelName = true;
		}).await;
	}

	onMount(() => {
		handleGetJSON();
	});
</script>

<nav class="flex flex-col flex-1 overflow-y-auto bg-white">
	<div class="bg-gray-500 flex items-center p-8 w-full font-sans text-3xl">Tealinux Installer</div>
</nav>

<main class="p-16">
	<div class="bg-gray-700 w-full grid-rows-1 grid place-items-center py-3 h-[40vh]">
		<img src="data:image/jpeg;base64,/9j/4AAQSkZJRgABAQAAAQABAAD/2wCEAAoHCBURFRgREhUSERgSGBISERIRERIREhERGBQaGRgYGBgcIS4lHB4rIRkYJjgmKy8xNTU1GiQ7QDs0Pzw0NTEBDAwMEA8QGhISGjQsIys0MTU6NDE0NDQ0NDE0NDQ0MTE0NDE0NDQxMT80NDQxMTE0MTY0NDQxNDQ0NDE0NDQ0NP/AABEIALcBEwMBIgACEQEDEQH/xAAbAAABBQEBAAAAAAAAAAAAAAADAAECBAUGB//EAD0QAAIBAgMDBg0CBgMBAAAAAAECAAMRBBIhBTFBIlFSYYHRBhMyQnFygpGSoaKxshQjM2LB4fDxFUNTB//EABsBAAMBAQEBAQAAAAAAAAAAAAABAgQDBQYH/8QAKxEAAgIBBAIBAwIHAAAAAAAAAAECEQMEEiExBUFRExRhcYEiIzJCkaGx/9oADAMBAAIRAxEAPwDYo0lyryV8lfNHNC+JXor8Ij4cclfVX7Qtp5LuxgRQXor8IkhSTor8IhDIEwtgRNJOivwiLxKdFfhEeSAjtjIiivRX4RJiinRX4REFhFEVsBJQTor8KyYoJ0E+ESaQkdsRAYdOgvwiT/Tp0F+ESSmShYA/06dFPhWLxCdBPhWEjEx7mAM0E6CfCsg1BOinwrDM0GTE2xgzRToJ8KyBop0U+EQzSEmykhvFJ0E+ERCgnQX4RIJXUkqLkjmtb3yb4pRoFsRvvczLPWRg6Vs1w0eSXaokKKdFPhWT8QnRT4Vgf+QN9wPXHGNvuseo75Mdb+GW/Hy+QviE6KfCIvEJ0U+ERUsSr9W+xH2IhDNePKpq0zLlwTxupAxRToJ8IkvEJ0E+FY0kDOikzkI0E6CfCsiaCdBPhWTDRGVYAvEJ0E+FZB6CdFfhEPeRaDYMrvQTor8IgGoJ0V+ES08E8lskqtRXor8Ig2or0V+ESw0CxkOxmPjaa5zyV4cBzCKPjfLPZ9hFOiYG/h/JX1V+0JB4fyF9VftCQa5EIwTCFitJoASiTAksskFjSGhKIRVkQsIBHQxxJWjCPeS0TQ4MWaNeMTACd4xMheK8QCMYxiYxMYxEyhjcWFut7W8o9XNLVepkUtzDS/E8JzwBclrrrvzG15k1M/7Uex43TKV5J9LoM2MDeRcEbmCnLLGzmYsS12vrwA7CZHBHLybAAWGltZbxFZSpB5N9LgXseeYqV1XB6WV0nGKKuJrqVD3ZA5yozI6o+tgQxUCxuLHcbi1xYzOGKNyDodd+momRitkk1zV8bV5SuuRtaCZ1yuQ9/J87LlGtpcexZgrFibWaxGp3gD3zZlxY41sfFE6GU5blkj0XsDjjmtfjw1t1zqMNWzpc26rc3H7iYWx9jllNROUwU5VYkIWtoDYXAvzCdRSwoSkqlQraFgCd/G1+HV6JOGL3prozeRyY5JxiuSuY15MrI2m+jwKEDHjRxGNCkHMcmQeIKIO0E5k2gTJYqItAtDtBsImBjY08s9n2EUWNHLPZ9hFOiQG/Q8hfVX7Qog6HkL6q/aFEbEOBHtEJMCCQEAskFkwI4EdDQgskIooqGIyMeRMKFQrxi0iYxk0FDlpHNFGMAoRaK8iRA4zECkjVD5gvbnO4D3kRxi20gfCKm061yEG4atxsTuHp75m5FVdS12JtbfvgdmVCyF2JZnJe2txfd6N80Gpkm+Xda083UpRyySdpM+o0T24IWq4sHTGXcbc1yb9sVWpcXOtt5BOphfF233ufNG6AZbHWwHDXjOS5NSpuylVS+trXtra5lWu4Q2W4sDbXW/P6ZbxOLCgAb9+7d1CYtWpe7tqOPVO0It9nbqLbOr2Bt3Iiq3mnKeFzv+xmmm1zUqIo3Mba9YnBvg65slDD13Y5HDmiypbW4zNoAbjUem+6dnsGmjPlBdnpIrPmAGVm0FyOJ1PZNrg4qLj0eDLPie7euaZvSJWFIjWnY8YEVjEQxEgyxUACReFZZBlklAmgSJYYQLCIQO0YiTtGYR0IxMaOWez7CPHxo5Z7PsIp0oDboeQvqr9hDQWHPIX1V+whRJJJLCLILJiUkOiYEe0ZY8YxCIxxGhQDRjJGMRFQEMsGwh4NhJYEIxEciK0AIkTnPCzFWRaYJGdgGK2I46MeA49k3mxAuVUhmXyluBb/ADSYy7OfEVAKg031D0Raze8/eVuWP+KXoh3KSjH2E2LgcyqQLBRa/BiN57uq01jhSLn5Sy9QIoVQFCiwA00EqPir6CfPzyOcmz6SDm0qVJGdjEIMyqtE6sxJJ3DnnQVXlDEuLEEAjd2TrjZtxTa4o5bEEm+a/fCbIwoq1aaP5Oe7DgwBFgfdNDEBH0AtbsF5LZjhHUjTKyk9hmpTpdGnK92N0uaO/Qyvs3Z6YdPF08x1Z2ZzmZ3Y3JJ/puAllI4noJ8HxUuxiJGTaRhQhpFpOMRAALCQYQxEgwiaArsINhDOIIxUBAiRIhbSBEaQjEx4/cbs+wjyWP8A4h9n8RFLA1MOeSvqr9pZWVqHkr6q/YQ6yUuRBFhQIJIZZYxwIoo4EAEIo4WIiAEYjHMiTACLGRvHaVcZilooXfcttBvZibKo5ySQO2KgJ4vELTXO5sNALAlmY7lUDUk8wlTEY5bgAsDa7LZeQetgT7h85n4DDYjHVVtZAmWo7eaquNADzAaaeUQxOlptv4OU6QOes1hysiqqu4F7sSQTqeNtOEuOOUlwv3MuTN8HMYzGutshBGmYsWsVza7tbidFg6gyDhcAnnJOpvM7EYGjVeyLkDEAKrOSoGhvm0Ol73lgUwhy5s51I57b/dqZg8jFxiotm7xP8yTlXXsNiaotM16/NxlitUF9fcOaZmJrgHk6dQ/z0Ty4RPqcMPVEqmJIO+Uq9e/H/cDVr3N5SqVeaaowN0MSRaevYb7yGGr66b/tp/qUbkm0s0UsRrvnVxVFuCSPStlV/GUkfnGvpGktmYvgo/7RToN8iBNszZilcUfF6uGzPJL5I3iiAinYyijR44WFAQIg2ljLIMsNoyo4giIdxBMsmgICMyyVo9oUIw9oL+43s/iIoTaA/cb2fxEU6bQOv/4xWRTu5KbvVEoVsA6btR850+GTkJ6ifiJJ6IM0yxxZFnHqLQqzer7OVuEz62zmXydZwlikirKiiSiyEbxaITmFj3iMkIzRgmDMiY5kWMBkWnF7YxrYuv8ApKJuE4jcz2bPd9ygKCb8wPPY9Ht16q0GNFWZ2K0wV1KF97dgBPVa5nOjZ6YRWCNndwqZwGAIC2Nr662v2yZOlbMmqzKC2rtmi221o01w2F/apUwFzjktVbczk9fNzb+rNxvhCzsSzFsw1ZSTp9humRVrBQRwUA6DUmxuBfjKSVFcXyW15ILNfrub/wCXlRyza74POpzbs7LwfxBqFqhJsOQgItqbX9wt2maldgi8kAfc98wMG5pU1QXBTeLWuzan0nXfDnFZyL8d/uni6hynkbbPuvHaL6WCKX6sVavfX+0zK7g6w+L3aWseuZ3Kv6JUI8Hu4oJKxqsAwh7b/nAOOM7I7JkX54ajzjU8eoQA1hqRjYpdHaeCVbMzj+VSfSD/AHnTmcn4Ei5qHqUfP/funWzTgjUD4/ydfcSr8EAIrSQEmFndGAGFhQI4WSyykhELQbCHIg2EKAqusCyyy8AwiaGAKyQWTjqIgMTaCfuN7P4iKF2gv7jez+Iil2I9Bw3kJ6ifiIaBw3kJ6ifiIabCCMi1O8JGAgBTq4UNwmfV2cRqs3bSJWQ4Rl2gOZemV3i0G06WrQDcJnYnZ3R0nGWFrodmOYO19JZrYZ03g+mAyaFjoADY85tunCVx5YSlSbMHFbVJfIGupSplVbEBSVW7dbEHs+eHj6+86He1wbjTQQlchHddFuAq66sCWN+s3JJ7JjbSqcm43HQeiZHJykeJOTlO2ZAxF2bgASF9AOlzBNWynQ2vu13NwtaM9Kxve3PpfWDq2VS7X10TXW19+nE8O0zbFJ9GqEVJpI6j9cGANxuA06huEcYm/HhMLY9FmBdt7AfIWtNAvl7JgyQipNI/SdDGX0Iua5ovM+mp9EFmlfxl9bwiVrDd/uRto1VQVrjdx3yv6ZIuWg3aNICIMKrdcATzw1JWNggzuSqovSdjZR6Lkdl5VXwRkmoRcn0jufAaj+3UqdNwgPOEBv8AUzDsnUSlsbZwwtBKAObItmbpudWbtJMuia4qlSPiM2R5MkpP2yQEkBGAkwJ0ijiJRCBYkEnaWAJhBsssMINxBgVGSDZZaYQDiRYysVjoJIiJRJsZl7QH7jez+Iii2h/Eb2fxEUsk7vDeQnqJ+IhpSwNa6J6qfiJaDXm0gnEDFFABwYzRCPABoisVo5gADEBQpZrACcr4Q4oKMotbU2A0Btxl3be20Sp4m45AzODZcxI0ykkA2nCeFG10RQS2TMeTTVg7nfckKT1adcw6vdJbYrsz5pP+lGFj6xar4xvJVbjmJuRu7ZnVmOVVPEF+xjcD3WlDEY0PWCs5XMcjKL8lTbyjw9E0cWAWZtwUWHNM7xShSZjliaaMnGV7HLu4k83+fcQ+A2c+IYM9wo8lT9z1yzsbY7V28c6kLfkLa1wN151iYZaa62HUOEWbUKC2Q79n13ivGQxxWTKufSM39D4sZVNrD0mUaygddt53zTxFTNe3b/numRXNv6TNFt9n1MLrkGWHCQNSQYyBadqG2WhWsLSAe8AGklNzDaTZYQ666zuPAjZKlf1Ti7ZmWlfcthlZvTqR75w+GptUdaai5YhQOsm09i2fhRRppSXcihR12Gp7TeVCNyPH8vncMSgu3/wO0SiPEJoR8yTWSEiJJZ0iIIsmJFZOUJjGRIkjImJjAMIFxLLwLzk2CKzLEBJtI3kWMx9ofxG9n8RFG2gf3G9n8RFOliOrwvkJ6qfiJYVyJVwzchPVT8RLCT0CC2lXnhAZSMmHy7zaIC4I85na/hlhsJcVHzMP+tBnc9g3ds4nanh3i8VdcOq4RD/2OQ1Qjq4Cc5TUex0el7U2zQwq5qrqvMt7sfQJxW0PDarWuuGXxSf+jgFz6BuHznB4rGIpzVHfEOeLsSL85lZ9q1azZKKZrdHRF62O4TPLJOXEVQVRd25tGqoJziqGPLWsgqKWPEX1E5TE16r2UslMHXLSQJv57a/OWsdjkpXBZcRU4gE+IQ+nzyPdBJSeqPGBS11BYqLqGtru0E1YVSSkTsvlIzRheRnAJFyCbbp1Xg9g0clXLOq2YqzE3b082m6Y5qFKQyjIWfIfSRvmt4LvnWpm4ZVHpNz/AEmbVSlLG30kz2tDhwrLGMlbavldP8HZYeoLgCwVebmHD/OuVcbVzNzD5WgsO/kgcAb++HqgZb6a3C+633vPDqmfQKKjKzNquALe/rmPiX1+U16yXlavTV6eXQNwPXwv/nPNGN0aXLbHgxS8jAVLqSG5JGhB01iWpfd7zoPfNewyfcK+Swp1h0G63GUqVS5sLNbeeHZOl8HNlnFVQuuRLGq40yr0Qec/3kyTXAfcQjBzb4R0PgPscFjiWGi3Snfzm3M3oG73zuhAYamqKEQBVUBVUaAAQ6y4xo+W1eolnyOT/b9B7RpIyM6oyik1kRHUy0JhlkwYJTJRsTJmMYopLAG8C0M8A5nIYFjAkwjSIWIZi7Q/iN7P4iPH2gP3G9n8RFGTR1eH8hPVT8RCLpKeAxSsia+av4iXJ6QiOIxGQXAvOE8LsXVYcqv4lN2VDZm6s3dO3fChvKJMGdmUiQSisRuLC5ETVoR4xQwdaqbYfDVKl/8AsZbC/PmaXavgXtFyMwp2IvpU0HUdJ7GlMKLKABzAWkmGkhY0OzwHHbFfCEmuucjdTRjlPrNzTA2ltas48WbUk/8AOkMqEdfSns3hTgw+YEA3nmeI2VyiCPRG1QVYPwM2tgsK2bF0PGt5rFVcDsM9d2P4d7PZQlMrTvpkZAg7p47U8HCVLrobqAvOSZbHgLiSLixHbKTCj0vwm8HMLj0L4Q00qXzFQcqv2eaeNxOGwmx6mFDh0anmKWLWsSC17EekQOC2BjqGi1Ci9V2t75uLhcS6ZKtbxgGqg0wtm9Mz58TlF1/j5PU0Gs2ZIrIlXVvtFVagHWdwPzMBicSbhBuLEr2DX5mO1Jk0Ya6iQGGLG/BbHq1+26eQopPk+rbUo3Fl9sOxsDYX32Pu+0rNhirrmHlHKv8AM19P6D3S9hKlzYb7KBbtH9PlNOjgwatFiwKhwSDYZXB5NjzMbC3PaEOZqPyZ82X6cbfo1Nh+CNCsvj6ystS5FzqUH8oIsD169U4v/wCubOoYR8PTwyCndKjOqg8oAqFZid50bWe20aq71t7rTy3/AOptSxWMw+HzKTTp1PG5SCy52TKrHhorG3C957ccaUVFej5LJqZTyOTfZwvgtsWri2CUxzM7EHKidI/0557HsvZ6YZBTpiwGpPnMx3sTxMv+CHg7T2dRKoxqGoQ7VDaxFtALeaB9zLAq0K7lKRJIBbOi3pGxFwG3E6g2HPOE8LfPsrNqdyUF0v8Ab+SssKI9XDsnlDtG4xgZyprhmYneRMUYy0A8cSF46mWhBFMKpgVhFgwCRXkbxmaQwBu0E0k5gyZAEGEWWMTHBiYzG2gP3G9n8RFH2j/Eb2fxEUuhUQwasAliRov2nR0K5UcqY+Bp6If5V+012W62m8Rap1Q24ws5erUem11PZwl/BbVvZXFuvhAVGxGO6MrA6iOTARzu2qdzunLYnZYZ8w06p220Uzb5mjDa6QAxaWz7ZL20bNqN9v8Ac6rZ6dXZ1QWHwqk8qwsLDtmhgqWU25vtEig9XCI4sQBfn3GUauxxvWaJEdGINt/vMdiOT2r4P5yDbybg20BFvlMldlmjcPuItcWItcb/AHfOeklA2l7H5TExWynZWdyQFJbxVOmjlkDbrsL5iObd1zhPBCTtrk24ddlhHbfBwrYSkm52W51OW5Gnp9Pvm74MmgQwdlc63p5rHITblp51+cXB3dUkMLSe5qUa9M2DWFKoqgEHKt3HlabgdNPRD4ahhVdWyszMiujOhUqAuTLay5WIbc2/XrEUNPji9yXI82ty5Y7W+BbZxuMoOWopTemV5FTOSyObAeMTTk63zAndunDbP2JmZ6rt42s7s7lLuWLHUjdOz/55WPLp4kNnUZP0laqEBJAcOisrAC1xfh2SyooUVuopJdS2QEUlC3FyVIFjc3I3iaTEZuzKWMdPFktTRLooc2UqDowQMbei86Twe2f+lRlzmoXc1GZrDUgCwA4aTLfabGp4tBUdkILhEFrOpyqzgcnVTvtNHA0cQ5d1rMuvIpVqChOOmYHMbc4iBo3hUuLEXlergwdV5J5uH9obCMXQFlyNqrpvswNjY8Qd4PMRLGWTKMZdisxnQpowt9j2yDTcyAixF/TKlfZ4PkcnqO7+04SxV0NMygZNTJVaLJ5QI6+B7ZFZPXYwoMleQEcmFgSzRmMa8RMhgCcwZMm0gZNAMTHWQkxCgMfaA/cb2fxEUW0D+43s/iIpVAWsNiqeReV5q+a3MOqXVx1O3lfS3dFFNpJSxWIp9L6W7pWSvTuOV9Ld0UUYGpTxyKNH7Mrd0Km2aW4nX1W7oooACxOMpkeV9Ld0pDF076N9Ld0aKAGguKp21Ya/yt3SxRx9JPOvc78rd0UUQB2x1I65/pfug6eLpk3zW9lu6PFABfq6QGr3HWr2tzbpYpbUp+SW+lt3u64ooAGXaNI7n+l+6McdTPn29l+6KKAhHG0un9L90cbQpjz/AKX7o8UAI/r6R876X7pL9fS3ZvpbuiigAl2hT4v9L6/KOcfS6f0v3RRQAcbQpdP6X7ox2jS6f0v3RRQAY46kdMwI5ijd0p1zROqvl6srkfbSKKJpMClUxSqbFh7m7oM46n0vpbuiimZxVlCGOp9L6W7ojjqfS+lu6KKKkANsdT6X0t3QbY6n0vpbuiihSGR/XU+l9Ld0muOp9L6W7oooUgMfaGLTxjcro8G6I6o8UUoD/9k=" alt="">
	</div>
</main>

{#if showModelName}
	<div
		class="flex flex-col justify-center p-5 gap-y-4 bg-slate-200 w-fit rounded-xl shadow-xl hover:scale-105 hover:translate-x-2 hover:translate-y-2 transition-all"
	>
		<h1 class="font-bold">Device Information</h1>
		<div class="flex justify-between">
			<p>test</p>
			<p>babababa</p>

			<h1 class="font-bold">
                {#if json.battery.capacity != null}
                    Battery : {json.battery.capacity}%
                {:else}
                    No Battery Found
                {/if}
            </h1>
			<h1 class="font-bold flex items-center">
				{#if json.online.status}
					online <span
						class="w-3 ml-2 aspect-square rounded-full bg-green-400 inline-block border border-slate-600"
					></span>
				{:else}
					offline <span
						class="w-3 ml-2 aspect-square rounded-full bg-red-500 inline-block border border-slate-600"
					></span>
				{/if}
			</h1>
		</div>
		<hr class="border border-slate-800" />
		<p>{json.model.systemProductName + ' - ' + json.model.systemVersion}</p>
		<table>
			<tr>
				<td>OS</td>
				<td>: {json.operatingSystems ? json.operatingSystems.join(', ') : ''}</td>
			</tr>
			<tr>
				<td>CPU</td>
				<td>: {json.lspci.cpu ? json.lspci.cpu : ''}</td>
			</tr>
			<tr>
				<td>VGA</td>
				<td>: {json.lspci.vga ? json.lspci.vga : ''}</td>
			</tr>
			<tr>
				<td>Memory Capacity</td>
				<td>: {json.memory.capacity ? json.memory.capacity : ''} Mb</td>
			</tr>
			<tr>
				<td>Memory Used</td>
				<td>: {json.memory.used ? json.memory.used : ''} Mb</td>
			</tr>
			<tr>
				<td>Memory Free</td>
				<td>: {json.memory.capacity - json.memory.used} Mb</td>
			</tr>
		</table>
	</div>
{/if}
<a href="/partitioning">Partitioning</a>
