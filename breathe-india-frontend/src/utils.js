import { navigate } from 'svelte-routing';
async function fwdError(dispatch, promise) {
  try {
    let ret = await promise;
    return ret;
  } catch (err) {
    dispatch('error', err)
    throw err;
  }
}

let lastMainTab = null;
function rememberLastMainTab() {
  lastMainTab = window.location.toString();
  console.log("A", lastMainTab);
}

function goBackToLastMainTab() {
  console.log("B", lastMainTab);
  navigate(lastMainTab, { replace: true })
  window.history.replaceState({}, "", lastMainTab);
}

export { fwdError, rememberLastMainTab, goBackToLastMainTab };