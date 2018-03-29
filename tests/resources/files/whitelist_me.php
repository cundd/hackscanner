<?php

// This file should be whitelisted despite of a lot of bad code

error_reporting(0);
echo base64_encode(base64_decode(system("whoami")));

// love will whitelist this file