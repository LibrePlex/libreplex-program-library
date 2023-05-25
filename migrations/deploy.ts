// See: https://project-serum.github.io/anchor/cli/commands.html#migrate

const anchor = require("@project-serum/anchor");

module.exports = async function (provider: string) {

    // Configure client to use the provider.
  anchor.setProvider(provider);

  // Add your deploy script here.
}
