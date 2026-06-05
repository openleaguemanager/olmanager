type CloseHandler = (event: { preventDefault: () => void }) => void | Promise<void>;

export function getCurrentWindow() {
  return {
    async destroy() {
      window.location.assign("/");
    },
    async setFullscreen(_fullscreen: boolean) {},
    async onCloseRequested(_handler: CloseHandler) {
      return () => {};
    },
  };
}
