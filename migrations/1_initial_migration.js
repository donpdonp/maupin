const Migrations = artifacts.require("Migrations");
const Snipe = artifacts.require("Snipe");

module.exports = function (deployer) {
  deployer.deploy(Migrations);
  deployer.deploy(Snipe);
};
