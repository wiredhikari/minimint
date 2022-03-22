{ lib, rustPlatform, fetchFromGitHub}:

rustPlatform.buildRustPackage rec {
  pname = "minimint";
  version = "master";

#  cargoLock = {
#     lockFile = ../Cargo.lock;
  
  # };

  checkType = "debug";
  src = fetchFromGitHub {
    owner = "fedimint";
    repo = "minimint";
    rev = "master";
    sha256 = "sha256-uoTKfzsm9VkjZPXGoQOCPBzxMTkP8zTSf48eiv5zgAA=";
  };

  cargoSha256 =  "sha256-1lZElXK9M895DURLID2KJdmCkJRTFNVC3meLFluO2WU=";

  buildInputs = [
    # build
   ];

  meta = with lib; {
    description = "Federated Mint Prototype";
    homepage = "https://github.com/fedimint/minimint";
    license = licenses.mit;
    maintainers = with maintainers; [  ];
  };
}
