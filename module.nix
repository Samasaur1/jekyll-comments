{ config, lib, pkgs, ... }:

with lib;

let cfg = config.services.jekyll-comments;

in {
  options.services.jekyll-comments = {
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

  config = mkIf cfg.enable {
    systemd.services.jekyll-comments = {
      description = "Jekyll Comments Server";

      # TODO: should have package option or put it in pkgs or something idr too sleepy
      script = let package = pkgs.callPackage ./. {}; in ''
        cd $STATE_DIRECTORY
        ${package}/bin/jekyll-comments --port ${toString cfg.port}
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
