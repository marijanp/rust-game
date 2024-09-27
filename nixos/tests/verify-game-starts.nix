{
  nixosTest,
  game,
  pkgs,
}:
nixosTest {
  name = "verify host configuration test";

  enableOCR = true;

  nodes = {
    client =
      { ... }:
      {
        imports = [
          "${pkgs.path}/nixos/tests/common/x11.nix"
          "${pkgs.path}/nixos/tests/common/user-account.nix"
        ];
        hardware.graphics.enable = true;
        environment.systemPackages = [ game ];
        test-support.displayManager.auto.user = "alice";
        environment.variables."XAUTHORITY" = "/home/alice/.Xauthority";
      };
  };

  testScript = ''
    start_all()

    client.wait_for_x()
    client.execute("su - alice -c game >&2 &")
    client.wait_for_window("App")
    client.wait_for_text("Awesome Game")
    client.screenshot("main_menu")
  '';
}
