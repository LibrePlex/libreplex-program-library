const path = require('path');
const programDir = path.join(__dirname, '../', 'programs/libreplex/');
const idlDir = path.join(__dirname, '..', 'target', 'idl');
const sdkDir = path.join(__dirname, '..', 'src', 'generated', 'libreplex');
const binaryInstallDir = path.join(__dirname, '.crates');

module.exports = {
    idlGenerator: 'anchor',
    programName: 'libreplex',
    programId: 'L1BRc7ZYjj7t9k7E5xbdnKy3KhaY6sTcJx4gAsqxUbh',
    idlDir,
    sdkDir,
    binaryInstallDir,
    programDir,
};
