{ config, lib, pkgs }:

with lib;
{
  options = {
    services.jekyll-comments = {
      enable = mkEnableOption "jekyll-comments";
      port = mkOption {
        type = types.port;
        default = 10113;
        example = 46264;
        description = "The port to listen on";
      };
      openFirewall = mkOption {
        type = types.bool;
        default = true;
        example = false;
        description = "Whether to automatically open the port the server runs on";
      };
    };
  };

  config = mkIf cfg.enable {
    systemd.services.jekyll-comments = {
      description = "Jekyll Comments Server";

      script = ''
        cd $STATE_DIRECTORY
        ${inputs.jekyll-comments.packages.${pkgs.system}.default}/bin/jekyll-comments --port ${cfg.port}
      '';

      serviceConfig = {
        DynamicUser = true;
        EnvironmentFile = "/etc/jekyll-comments-env";
        StateDirectory = "jekyll-comments";

        PrivateDevices = true;
        PrivateMounts = true;
        PrivateUsers = true;
        ProtectControlGroups = true;
        ProtectHome = true;
        ProtectHostname = true;
        ProtectKernelLogs = true;
        ProtectKernelModules = true;
        ProtectKernelTunables = true;
      };

      wantedBy = [ "multi-user.target" ];
      after = [ "network-online.target" ];
      wants = [ "network-online.target" ];
    };
    networking.firewall.allowedTCPPorts = mkIf cfg.openFirewall [ cfg.port ];
  };
}
