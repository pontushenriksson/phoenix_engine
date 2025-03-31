document.addEventListener('DOMContentLoaded', () => {
  const video = document.querySelector('.gameShowcaseVideo');
  const playBtn = document.getElementById('playBtn');
  const pauseBtn = document.getElementById('pauseBtn');
  const stopBtn = document.getElementById('stopBtn');

  // Ensure the video is paused and muted by default
  video.mute = true;
  video.pause();

  playBtn.style.display = 'inline-block';
  pauseBtn.style.display = 'none';
  stopBtn.style.display = 'none';

  // When the Play button is clicked:
  playBtn.addEventListener('click', () => {
    video.play();

    // Hide play button, show pause and stop.
    playBtn.style.display = 'none';
    pauseBtn.style.display = 'inline-block';
    stopBtn.style.display = 'inline-block';
  });

  // When the Pause button is clicked:
  pauseBtn.addEventListener('click', () => {
    video.pause();

    // Show play button (to resume) and keep the stop button visible.
    playBtn.style.display = 'inline-block';
    pauseBtn.style.display = 'none';
    stopBtn.style.display = 'inline-block';
  });

  // When the Stop button is clicked:
  stopBtn.addEventListener('click', () => {
    video.pause();
    video.currentTime = 0;

    // Reset to the default state: only play visible.
    playBtn.style.display = 'inline-block';
    pauseBtn.style.display = 'none';
    stopBtn.style.display = 'none';
  });

  // When the video ends, reset to the default state.
  video.addEventListener('ended', () => {
    video.currentTime = 0;
    playBtn.style.display = 'inline-block';
    pauseBtn.style.display = 'none';
    stopBtn.style.display = 'none';
  });
});
