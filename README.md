<h1 align="center">
  Log Secure Deserializer
</h1>

# Purpose for application
LSD can convert security log files (/var/log/secure) from human readable logs to jsons, these jsons can be used for analysising log-in attempts in Grafana or other statistics apps.

The logs generated from /var/log/secure are often filled with megabytes of useless information, this program aims to generate machine readable smaller log files.

# Goals
LSD will be able to generate jsons showing
- Invalid usernames for log-in attempts (InvalidUser.txt)
- invalid passwords for log-in attempts (InvalidPassword.txt)
- invalid keys for login attempts (InvalidKey.txt)
- non sudoers attempting to use sudo (InvalidSudo.txt)



![LGPLv3 Badge](/README_RESOURCES/LGPLv3%20Logo.svg)
