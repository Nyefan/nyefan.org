function updateAndMaintainLayout() {
  const maxWidth = 1.2 * window.screen.height;
  document.documentElement.style.setProperty('--site-max-width', maxWidth + 'px');
  if (window.innerWidth < maxWidth) {
    document.documentElement.classList.add('when-viewport-is-narrow');
  } else {
    document.documentElement.classList.remove('when-viewport-is-narrow');
  }
}

window.addEventListener('resize', updateAndMaintainLayout);
updateAndMaintainLayout();