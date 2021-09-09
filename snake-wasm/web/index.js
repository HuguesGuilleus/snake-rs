var control = 0,
	width = 10,
	height = 10,
	stoped = true,
	ended = false;

async function main() {
	const m = await WebAssembly.instantiateStreaming(fetch('snake.wasm'), {
		env: {
			random: function () {
				return Math.random() * 0xFFFFFFFF;
			},
			screen(ptr, len) {
				document.body.innerText = new TextDecoder()
					.decode(new Uint8Array(m.instance.exports.memory.buffer, ptr, len))
					.replaceAll(new RegExp(`(.{${width}})`, "g"), "$1\n");
			}
		},
	});
	const {
		init,
		step
	} = m.instance.exports;

	async function run() {
		if (stoped) {
			return;
		}
		while (!stoped && step(control)) {
			await new Promise(resolve => setInterval(resolve, 100));
		}
		if (!stoped) {
			stoped = true;
			ended = true;
		}
	}

	function start() {
		control = 0;
		size();
		init(width, height);
		stoped = false;
		ended = false;
		run();
	}

	window.addEventListener('keydown', e => {
		switch (e.key) {
		case 'ArrowLeft':
			control = 1;
			break;
		case 'ArrowRight':
			control = 2;
			break;
		case 'ArrowUp':
			control = 3;
			break;
		case 'ArrowDown':
			control = 4;
			break;
		case ' ':
		case 'Escape':
			if (ended) {
				start();
			} else if (stoped) {
				stoped = false;
				run();
			} else {
				stoped = true;
				document.body.innerText = "[PAUSE]";
			}
		}
	});

	window.addEventListener('resize', start);
	start();
}
document.readyState == 'loading' ? document.addEventListener('DOMContentLoaded', main, {
	once: true
}) : main();

// Set with an height global varibale with the windows size in character unit.
// This function erase document.body.
function size() {
	document.body.innerHTML = "<span>#<br>#</span>";
	const r = document.querySelector('span').getBoundingClientRect();

	width = Math.trunc(window.innerWidth / r.width);
	height = Math.trunc(window.innerHeight * 2 / r.height);
}