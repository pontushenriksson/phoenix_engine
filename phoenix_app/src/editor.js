document.addEventListener('DOMContentLoaded', () => {
  const video = document.querySelector('.gameShowcaseVideo');
  const playBtn = document.getElementById('playBtn');
  const stopBtn = document.getElementById('stopBtn');

  // Ensure the video is paused by default.
  video.pause();

  playBtn.addEventListener('click', () => {
    video.play();
  });

  stopBtn.addEventListener('click', () => {
    video.pause();
    video.currentTime = 0;
  });

  // Reset the video when it ends.
  video.addEventListener('ended', () => {
    video.currentTime = 0;
  });
});
