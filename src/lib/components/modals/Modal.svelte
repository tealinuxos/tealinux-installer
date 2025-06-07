<script>
	import { modalStore, closeModal } from '$lib/stores/modalStore';

	let modal;

	$: {
        modal = $modalStore;
        onMount();
    }

    let interval = null;
    let disabled = true;
    let countdown = null;

	const handleConfirm = () => {
        clearInterval(interval);
        interval = null;
        if (modal.onConfirm) modal.onConfirm();
        closeModal();
	};

	const handleCancel = () => {
        clearInterval(interval);
        interval = null;
		if (modal.onCancel) modal.onCancel();
		closeModal();
	};

    const onMount = () => {
        countdown = modal?.countdown ?? 0;
        disabled = true;
        if (!interval && countdown !== 0) {
            disabled = true;
            interval = setInterval(() => {
                if (countdown > 0) {
                    countdown -= 1;
                } 
                else if (countdown === 0) {
                    clearInterval(interval);
                    disabled = false;
                    interval = null;
                    countdown = -1;
                }
            }, 1000)
        }

        if (!countdown) disabled = false;
    };
</script>

{#if modal.isOpen}
	{#if modal.type === 'error'}
		<!-- Custom Error Modal (your design) -->
		<div class="absolute inset-0 z-50 flex items-center justify-center bg-black/50">
			<div
				class="inline-flex p-[27px_35px] flex-col justify-center items-center gap-2.5 rounded-[20px] border-2 border-[#F00] bg-[#101010] max-w-[80%]"
			>
				<!-- Warning Icon -->
				<svg
					width="66"
					height="61"
					viewBox="0 0 66 61"
					fill="none"
					xmlns="http://www.w3.org/2000/svg"
				>
					<path
						d="M35.2019 16.8826C35.2019 15.6664 34.216 14.6805 32.9998 14.6805C31.7836 14.6805 30.7977 15.6664 30.7977 16.8826V34.4991C30.7977 35.7153 31.7836 36.7012 32.9998 36.7012C34.216 36.7012 35.2019 35.7153 35.2019 34.4991V16.8826Z"
						fill="#FF0000"
					/>
					<path
						fill-rule="evenodd"
						clip-rule="evenodd"
						d="M39.8666 1.45957C35.4956 -0.486524 30.5041 -0.486524 26.1331 1.45957C23.4001 2.67638 21.1931 5.00826 18.914 8.23406C16.6422 11.4497 14.0979 15.8565 10.8109 21.5497L10.6908 21.7577C7.40387 27.4509 4.85956 31.8577 3.21069 35.4331C1.5566 39.0197 0.640608 42.0969 0.95332 45.0722C1.45345 49.8306 3.9492 54.1534 7.82007 56.9657C10.2403 58.7241 13.3633 59.4695 17.2965 59.8303C21.2172 60.19 26.3057 60.19 32.8795 60.19H33.1199C39.6938 60.19 44.7824 60.19 48.7032 59.8303C52.6363 59.4695 55.7593 58.7241 58.1795 56.9657C62.0504 54.1534 64.5462 49.8306 65.0463 45.0722C65.359 42.0969 64.443 39.0197 62.7889 35.4331C61.14 31.8577 58.5957 27.4508 55.3087 21.7576L55.1885 21.5494C51.9017 15.8564 49.3574 11.4496 47.0856 8.23406C44.8065 5.00826 42.5995 2.67638 39.8666 1.45957ZM27.9244 5.48296C31.1551 4.04454 34.8445 4.04454 38.0752 5.48296C39.6998 6.20628 41.3405 7.73502 43.4886 10.7754C45.63 13.8063 48.0752 18.0371 51.4346 23.8558C54.7941 29.6745 57.2354 33.9074 58.7896 37.2775C60.3486 40.6579 60.8522 42.8432 60.6663 44.6118C60.2966 48.1289 58.4519 51.324 55.5908 53.4027C54.1521 54.448 52.0079 55.1045 48.3008 55.4446C44.6052 55.7836 39.7187 55.7859 32.9998 55.7859C26.2809 55.7859 21.3944 55.7836 17.6988 55.4446C13.9918 55.1045 11.8475 54.448 10.4088 53.4027C7.54769 51.324 5.703 48.1289 5.33334 44.6118C5.14745 42.8432 5.65102 40.6579 7.21002 37.2775C8.7642 33.9074 11.2055 29.6745 14.565 23.8558C17.9244 18.0371 20.3696 13.8063 22.511 10.7754C24.6591 7.73502 26.2998 6.20628 27.9244 5.48296Z"
						fill="#FF0000"
					/>
					<path
						d="M35.936 43.3074C35.936 44.929 34.6215 46.2435 32.9999 46.2435C31.3784 46.2435 30.0638 44.929 30.0638 43.3074C30.0638 41.6859 31.3784 40.3713 32.9999 40.3713C34.6215 40.3713 35.936 41.6859 35.936 43.3074Z"
						fill="#FF0000"
					/>
				</svg>

				<!-- Title -->
				<h2
					class="text-[#FFF] text-center font-['Plus_Jakarta_Sans'] text-[24px] font-extrabold leading-none mt-4"
				>
					{modal.title}
				</h2>

				<!-- Message -->
				<p
					class="text-[#FFF] text-center font-['Plus_Jakarta_Sans'] text-[14px] font-extralight leading-normal tracking-[-0.56px] mt-2 max-w-[300px]"
				>
					{@html modal.content}
				</p>

				<!-- Buttons -->
				<div class="flex gap-4 mt-6">
					{#if modal.showCancel}
						<button
							on:click={handleCancel}
							class="text-[#A72626] font-['Poppins'] text-[20px] font-light leading-none capitalize px-[15px] py-[8px] flex flex-col items-center gap-[18px] rounded-[23px] border-[1.3px] border-[#F00] bg-[#101010] active:shadow-[0_0_9px_0_#FF453A]"
						>
							Cancel
						</button>
					{/if}
                    {#key countdown}
                        <button
                            on:click={handleConfirm}
                            disabled={disabled}
                            class="text-[#A72626] font-['Poppins'] text-[20px] font-light leading-none capitalize px-[15px] py-[8px] flex flex-col items-center gap-[18px] rounded-[23px] border-[1.3px] border-[#F00] bg-[#101010] active:shadow-[0_0_9px_0_#FF453A]"
                        >
                            Confirm { countdown >= 0 ? `(${countdown})` : ''}
                        </button>
                    {/key}
				</div>
			</div>
		</div>
	{:else}
		<!-- Original Modal (all other types) -->
		<div class="fixed inset-0 z-50 overflow-y-auto">
			<div
				class="flex items-center justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0"
			>
				<!-- Background overlay -->
				<div class="fixed inset-0 transition-opacity" on:click={closeModal}>
					<div class="absolute inset-0 bg-gray-500 opacity-75"></div>
				</div>

				<!-- Modal container -->
				<div
					class="inline-block align-bottom bg-white rounded-lg text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:align-middle sm:max-w-lg sm:w-full"
					role="dialog"
					aria-modal="true"
					aria-labelledby="modal-headline"
				>
					<div class="bg-white px-4 pt-5 pb-4 sm:p-6 sm:pb-4">
						<div class="sm:flex sm:items-start">
							<!-- Icon based on modal type -->
							{#if modal.type === 'success'}
								<div
									class="mx-auto flex-shrink-0 flex items-center justify-center h-12 w-12 rounded-full bg-green-100 sm:mx-0 sm:h-10 sm:w-10"
								>
									<svg
										class="h-6 w-6 text-green-600"
										fill="none"
										viewBox="0 0 24 24"
										stroke="currentColor"
									>
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M5 13l4 4L19 7"
										/>
									</svg>
								</div>
							{:else if modal.type === 'warning'}
								<div
									class="mx-auto flex-shrink-0 flex items-center justify-center h-12 w-12 rounded-full bg-yellow-100 sm:mx-0 sm:h-10 sm:w-10"
								>
									<svg
										class="h-6 w-6 text-yellow-600"
										fill="none"
										viewBox="0 0 24 24"
										stroke="currentColor"
									>
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
										/>
									</svg>
								</div>
							{:else}
								<div
									class="mx-auto flex-shrink-0 flex items-center justify-center h-12 w-12 rounded-full bg-blue-100 sm:mx-0 sm:h-10 sm:w-10"
								>
									<svg
										class="h-6 w-6 text-blue-600"
										fill="none"
										viewBox="0 0 24 24"
										stroke="currentColor"
									>
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
										/>
									</svg>
								</div>
							{/if}

							<div class="mt-3 text-center sm:mt-0 sm:ml-4 sm:text-left">
								<h3 class="text-lg leading-6 font-medium text-gray-900" id="modal-headline">
									{modal.title}
								</h3>
								<div class="mt-2">
									<p class="text-sm text-gray-500">
										{@html modal.content}
									</p>
								</div>
							</div>
						</div>
					</div>
					<div class="bg-gray-50 px-4 py-3 sm:px-6 sm:flex sm:flex-row-reverse">
						<button
							type="button"
							class="w-full inline-flex justify-center rounded-md border border-transparent shadow-sm px-4 py-2 bg-blue-600 text-base font-medium text-white hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 sm:ml-3 sm:w-auto sm:text-sm"
							on:click={handleConfirm}
						>
							OK
						</button>
						{#if modal.showCancel}
							<button
								type="button"
								class="mt-3 w-full inline-flex justify-center rounded-md border border-gray-300 shadow-sm px-4 py-2 bg-white text-base font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 sm:mt-0 sm:ml-3 sm:w-auto sm:text-sm"
								on:click={closeModal}
							>
								Cancel
							</button>
						{/if}
					</div>
				</div>
			</div>
		</div>
	{/if}
{/if}
