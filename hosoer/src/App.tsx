import type { Component } from "solid-js";
import { createEffect, createSignal } from "solid-js";
import { getHeartbeat } from "./apis/index";

const App: Component = () => {
	const [heartbeat, setHeartbeat] = createSignal<string>("50");
	createEffect(async () => {
		const interval = setInterval(async () => {
			setHeartbeat(await getHeartbeat());
		}, 1000);
		return () => clearInterval(interval);
	});

	return (
		<div class="w-screen h-screen overflow-hidden">
			<div class="flex-col justify-start h-full w-full bg-cerise-red-50 overflow-hidden">
				<div class="flex-row justify-end items-center h-8" />
				<div class="grid grid-cols-1 grid-rows-9 sm:grid-cols-9 sm:grid-rows-8 w-full h-full">
					<div class="shadow-xl col-start-1 col-span-1 row-start-3 row-span-3 mx-4 sm:col-start-4 sm:col-span-3 sm:row-start-3 sm:row-span-3 bg-cerise-red-300 rounded-xl">
						<div class="flex justify-evenly content-center items-center h-full w-full">
							<div class="animate-pulse items-center">
								{/* biome-ignore lint/a11y/noSvgWithoutTitle: <explanation> */}
								<svg
									xmlns="http://www.w3.org/2000/svg"
									class="icon icon-tabler icon-tabler-heartbeat"
									width="100"
									height="100"
									viewBox="0 0 24 24"
									stroke-width="1.5"
									stroke="#b21e4b"
									fill="none"
									stroke-linecap="round"
									stroke-linejoin="round"
								>
									<path stroke="none" d="M0 0h24v24H0z" fill="none" />
									<path d="M19.5 13.572l-7.5 7.428l-2.896 -2.868m-6.117 -8.104a5 5 0 0 1 9.013 -3.022a5 5 0 1 1 7.5 6.572" />
									<path d="M3 13h2l2 3l2 -6l1 3h3" />
								</svg>
							</div>

							<span class="shadow-md font-mono text-6xl text-cerise-red-100 bg-cerise-red-200/20 backdrop-blur-lg px-3 pt-3 rounded-md">
								<div class="">{heartbeat()}</div>
							</span>
						</div>
					</div>
				</div>
			</div>
		</div>
	);
};

export default App;
