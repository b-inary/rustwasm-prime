import './style.css'

let worker: Worker;
let running = false;

const $input = document.querySelector('#input')! as HTMLTextAreaElement;
const $start_button = document.querySelector('#start-button')! as HTMLButtonElement;
const $cancel_button = document.querySelector('#cancel-button')! as HTMLButtonElement;
const $spinner = document.querySelector('#spinner')! as HTMLSpanElement;
const $result = document.querySelector('#result')! as HTMLSpanElement;

// Handle enter key
$input.onkeydown = ev => {
  if (ev.key === 'Enter') {
    if (!ev.shiftKey) {
      ev.preventDefault();
      if (!ev.repeat && !running) $start_button.click();
    }
  }
}

// Start computation
$start_button.onclick = ev => {
  running = true;
  worker = new Worker('./worker', { type: 'module' });

  $spinner.classList.remove('hidden');
  $start_button.classList.add('hidden');
  $cancel_button.classList.remove('hidden');
  $result.textContent = 'Computing...';

  const input = $input.value;
  worker.onmessage = onComputationFinished;
  worker.postMessage(input);
};

// Cancel computation
$cancel_button.onclick = ev => {
  if (!running) return;
  running = false;
  worker.terminate();
  $spinner.classList.add('hidden');
  $start_button.classList.remove('hidden');
  $cancel_button.classList.add('hidden');
  $result.textContent = 'Result: -';
}

// On computation finished
function onComputationFinished(ev: MessageEvent) {
  running = false;
  worker.terminate();
  $spinner.classList.add('hidden');
  $start_button.classList.remove('hidden');
  $cancel_button.classList.add('hidden');
  $result.textContent = ev.data;
}
