// can use `process.env.SECRET_MNEMONIC` or `process.env.SECRET_PRIV_KEY`
// to populate secret in CI environment instead of hardcoding
const wallets = require("./.wallets.json");

module.exports = {
  terra_wallet_1: {
    mnemonic: wallets.wallets[0].mnemonic
  }
};
