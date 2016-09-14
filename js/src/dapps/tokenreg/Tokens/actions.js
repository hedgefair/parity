const sha3 = window.parity.api.format.sha3;

export const SET_TOKENS_LOADING = 'SET_TOKENS_LOADING';
export const setTokensLoading = (isLoading) => ({
  type: SET_TOKENS_LOADING,
  isLoading
});

export const SET_TOKEN_COUNT = 'SET_TOKEN_COUNT';
export const setTokenCount = (tokenCount) => ({
  type: SET_TOKEN_COUNT,
  tokenCount
});

export const SET_TOKEN_DATA = 'SET_TOKEN_DATA';
export const setTokenData = (index, tokenData) => ({
  type: SET_TOKEN_DATA,
  index, tokenData
});

export const SET_TOKEN_META = 'SET_TOKEN_META';
export const setTokenMeta = (index, meta) => ({
  type: SET_TOKEN_META,
  index, meta
});

export const SET_TOKEN_LOADING = 'SET_TOKEN_LOADING';
export const setTokenLoading = (index, isLoading) => ({
  type: SET_TOKEN_LOADING,
  index, isLoading
});

export const loadTokens = () => (dispatch, getState) => {
  console.log('loading tokens...');

  let state = getState();
  let contractInstance = state.status.contract.instance;

  dispatch(setTokensLoading(true));

  contractInstance
    .tokenCount
    .call()
    .then((count) => {
      let tokenCount = parseInt(count);
      console.log(`token count: ${tokenCount}`);
      dispatch(setTokenCount(tokenCount));

      for (let i = 0; i < tokenCount; i++) {
        dispatch(loadToken(i));
      }

      dispatch(setTokensLoading(false));
    })
    .catch((e) => {
      console.error('loadTokens error', e);
    });
};

export const loadToken = (index) => (dispatch, getState) => {
  console.log('loading token', index);

  let state = getState();
  let contractInstance = state.status.contract.instance;

  dispatch(setTokenLoading(index, true));

  contractInstance
    .token
    .call({}, [ parseInt(index) ])
    .then((result) => {
      let data = {
        index: parseInt(index),
        address: result[0],
        tla: result[1],
        base: result[2].toNumber(),
        name: result[3],
        owner: result[4]
      };

      console.log(`token loaded: #${index}`, data);
      dispatch(setTokenData(index, data));
      dispatch(setTokenLoading(index, false));
    })
    .catch((e) => {
      console.error('loadToken #${index} error', e);
    });
};

export const queryTokenMeta = (index, query) => (dispatch, getState) => {
  console.log('loading token meta', index, query);

  let state = getState();
  let contractInstance = state.status.contract.instance;

  let key = sha3(query);

  dispatch(setTokenLoading(index, true));

  contractInstance
    .meta
    .call({}, [ index, key ])
    .then((value) => {
      let meta = {
        key, query, value
      };

      console.log(`token meta loaded: #${index}`, value);
      dispatch(setTokenMeta(index, meta));
      dispatch(setTokenLoading(index, false));
    })
    .catch((e) => {
      console.error('loadToken #${index} error', e);
    });
}