{ lib, fetchFromGitHub, rustPlatform }:

rustPlatform.buildRustPackage rec {
    pname = "minimint";
    src = fetchFromGitHub {
    owner = "elsirion";
    repo = minimint;
  };


  meta = with lib; {
    description = "Federated Mint Prototype,This is an experimental implementation of a federated Chaumian bank. ";
    homepage = "https://github.com/fedimint/minimint";
    license = licenses.MIT;
    maintainers = [ maintainers.elsirion ];
  };
}
