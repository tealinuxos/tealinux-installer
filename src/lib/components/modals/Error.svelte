<script>
    let { 
        title = "Warning", 
        message = "This configure will reset all your data. Please backup your data before run.",
        showCancel = true,
        onConfirm = null,
        onCancel = null
    } = $props();

    let showModal = $state(true);

    const confirm = () => {
        showModal = false;
        // Tambahkan logika konfirmasi di sini
    };

    const cancel = () => {
        showModal = false;
        // Tambahkan logika cancel di sini
    };
</script>

{#if showModal}
    <div class="absolute inset-0 z-50 flex items-center justify-center bg-black/50">
        <div class="inline-flex p-[27px_35px] flex-col justify-center items-center gap-2.5 rounded-[20px] border-2 border-[#F00] bg-[#101010] max-w-[80%]">
            <!-- Warning Icon -->
            <svg width="66" height="61" viewBox="0 0 66 61" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M35.2019 16.8826C35.2019 15.6664 34.216 14.6805 32.9998 14.6805C31.7836 14.6805 30.7977 15.6664 30.7977 16.8826V34.4991C30.7977 35.7153 31.7836 36.7012 32.9998 36.7012C34.216 36.7012 35.2019 35.7153 35.2019 34.4991V16.8826Z" fill="#FF0000"/>
                <path fill-rule="evenodd" clip-rule="evenodd" d="M39.8666 1.45957C35.4956 -0.486524 30.5041 -0.486524 26.1331 1.45957C23.4001 2.67638 21.1931 5.00826 18.914 8.23406C16.6422 11.4497 14.0979 15.8565 10.8109 21.5497L10.6908 21.7577C7.40387 27.4509 4.85956 31.8577 3.21069 35.4331C1.5566 39.0197 0.640608 42.0969 0.95332 45.0722C1.45345 49.8306 3.9492 54.1534 7.82007 56.9657C10.2403 58.7241 13.3633 59.4695 17.2965 59.8303C21.2172 60.19 26.3057 60.19 32.8795 60.19H33.1199C39.6938 60.19 44.7824 60.19 48.7032 59.8303C52.6363 59.4695 55.7593 58.7241 58.1795 56.9657C62.0504 54.1534 64.5462 49.8306 65.0463 45.0722C65.359 42.0969 64.443 39.0197 62.7889 35.4331C61.14 31.8577 58.5957 27.4508 55.3087 21.7576L55.1885 21.5494C51.9017 15.8564 49.3574 11.4496 47.0856 8.23406C44.8065 5.00826 42.5995 2.67638 39.8666 1.45957ZM27.9244 5.48296C31.1551 4.04454 34.8445 4.04454 38.0752 5.48296C39.6998 6.20628 41.3405 7.73502 43.4886 10.7754C45.63 13.8063 48.0752 18.0371 51.4346 23.8558C54.7941 29.6745 57.2354 33.9074 58.7896 37.2775C60.3486 40.6579 60.8522 42.8432 60.6663 44.6118C60.2966 48.1289 58.4519 51.324 55.5908 53.4027C54.1521 54.448 52.0079 55.1045 48.3008 55.4446C44.6052 55.7836 39.7187 55.7859 32.9998 55.7859C26.2809 55.7859 21.3944 55.7836 17.6988 55.4446C13.9918 55.1045 11.8475 54.448 10.4088 53.4027C7.54769 51.324 5.703 48.1289 5.33334 44.6118C5.14745 42.8432 5.65102 40.6579 7.21002 37.2775C8.7642 33.9074 11.2055 29.6745 14.565 23.8558C17.9244 18.0371 20.3696 13.8063 22.511 10.7754C24.6591 7.73502 26.2998 6.20628 27.9244 5.48296Z" fill="#FF0000"/>
                <path d="M35.936 43.3074C35.936 44.929 34.6215 46.2435 32.9999 46.2435C31.3784 46.2435 30.0638 44.929 30.0638 43.3074C30.0638 41.6859 31.3784 40.3713 32.9999 40.3713C34.6215 40.3713 35.936 41.6859 35.936 43.3074Z" fill="#FF0000"/>
            </svg>

            <!-- Title -->
            <h2 class="text-[#FFF] text-center font-['Plus_Jakarta_Sans'] text-[24px] font-extrabold leading-none mt-4">
                {title}
            </h2>

            <!-- Message -->
            <p class="text-[#FFF] text-center font-['Plus_Jakarta_Sans'] text-[14px] font-extralight leading-normal tracking-[-0.56px] mt-2 max-w-[300px]">
                {message}
            </p>

            <!-- Buttons -->
            <div class="flex gap-4 mt-6">
                {#if showCancel}
                <button 
                    onclick={onCancel}
                    class="text-[#A72626] font-['Poppins'] text-[20px] font-light leading-none lowercase px-[15px] py-[8px] flex flex-col items-center gap-[18px] rounded-[23px] border-[1.3px] border-[#F00] bg-[#101010] active:shadow-[0_0_9px_0_#FF453A]"
                >
                    cancel
                </button>

               
                {/if}
                <button 
                    onclick={onConfirm}
                    class="text-[#A72626] font-['Poppins'] text-[20px] font-light leading-none lowercase px-[15px] py-[8px] flex flex-col items-center gap-[18px] rounded-[23px] border-[1.3px] border-[#F00] bg-[#101010] active:shadow-[0_0_9px_0_#FF453A]"
                >
                    confirm
                </button>

             
                

            </div>
        </div>
    </div>
{/if}
