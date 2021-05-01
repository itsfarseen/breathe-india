async function fwdError(dispatch, promise) {
  try {
    let ret = await promise;
    return ret;
  } catch (err) {
    dispatch('error', err)
    throw err;
  }
}

export { fwdError };