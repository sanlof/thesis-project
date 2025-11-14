#!/bin/bash
set -euo pipefail

# -----------------------------------------------------------------------------
# SAFE INTRUSIVE NMAP SCAN (multi-gate, conservative)
# -----------------------------------------------------------------------------
# DESCRIPTION (read this):
# This script performs conservative information-gathering scans by default
# and will ONLY run intrusive tests if:
#  1) you have written authorization and explicitly confirm it,
#  2) you confirm these are your systems,
#  3) you confirm you are running against a staging/test environment (not prod),
#  4) you confirm you have a valid backup/snapshot (local file or snapshot id),
#  5) you type the explicit acknowledgement phrase:
#       I_ACCEPT_RISK_AND_HAVE_STAGING_AND_BACKUP
#
# Intrusive tests are intended for staging/test replicas only. They can
# cause service disruption, create or modify data, or trip detection systems.
# Do NOT run intrusive tests against production or third-party infrastructure.
#
# Keep all comments in English. Review and edit the TARGET, PORTS and other
# defaults as appropriate before running.
# -----------------------------------------------------------------------------

#################################
# Helper functions
#################################

# Check whether an IP is private/local (returns 0 if private)
is_private_ip() {
  local ip="$1"
  # IPv4 checks: 127.*, 10.*, 172.16-31.*, 192.168.*, 169.254.*
  # Handle basic IPv4 only for safety gating
  if [[ "$ip" =~ ^127\. ]] || [[ "$ip" =~ ^10\. ]] || [[ "$ip" =~ ^192\.168\. ]] || [[ "$ip" =~ ^169\.254\. ]]; then
    return 0
  fi
  if [[ "$ip" =~ ^172\.(1[6-9]|2[0-9]|3[0-1])\. ]]; then
    return 0
  fi
  return 1
}

# Resolve target to first address (IPv4 preferred)
resolve_target_ip() {
  local tgt="$1"
  # Use getent if available for portability, fallback to host/dig if not
  if command -v getent >/dev/null 2>&1; then
    getent hosts "$tgt" | awk '{print $1; exit}'
  elif command -v host >/dev/null 2>&1; then
    host "$tgt" | awk '/has address/ {print $4; exit}'
  else
    # best-effort fallback using ping (may be blocked)
    ping -c1 -W1 "$tgt" 2>/dev/null | head -n1 | sed -n 's/.*(\([0-9.]\+\)).*/\1/p'
  fi
}

# Abort on ctrl-c and show message
trap 'echo; echo "Interrupted by user. Exiting."; exit 1' INT

#################################
# User confirmations (legal)
#################################

echo "⚠️  LEGAL WARNING ⚠️"
echo "This script performs security testing that may be illegal without authorization."
echo "You may ONLY proceed if:"
echo "  ✓ You own or have written authorization for the target systems"
echo "  ✓ You are testing in a staging/test environment or have explicit approval to test production"
echo "  ✓ You have backups/snapshots and a rollback plan"
echo ""
read -p "Do you have written authorization to test the target(s)? (type 'YES' to continue): " CONFIRM_AUTH
if [ "$CONFIRM_AUTH" != "YES" ]; then
  echo "Authorization not confirmed. Aborting."
  exit 1
fi

read -p "Are these YOUR services (you are authorized owner/operator)? (type 'YES' to continue): " CONFIRM_OWNER
if [ "$CONFIRM_OWNER" != "YES" ]; then
  echo "Ownership not confirmed. Aborting."
  exit 1
fi

#################################
# Target and defaults
#################################

TARGET="${1:-localhost}"         # default to localhost
# Conservative default ports: only app HTTP ports
PORTS="8000,8001"
TIMESTAMP=$(date +"%Y-%m-%d_%H-%M-%S")
RESULTS_DIR="nmap-results/scan_${TIMESTAMP}"

echo ""
echo "Target: ${TARGET}"
echo "Ports:  ${PORTS}"
echo "Results directory will be: ${RESULTS_DIR}"
echo ""

#################################
# Environment gating (staging/prod check)
#################################

# Prefer the operator to explicitly state environment, do not guess.
read -p "Are you running these scans against a STAGING or TEST environment (type 'staging' to confirm): " ENV_CONFIRM
if [ "$ENV_CONFIRM" != "staging" ]; then
  echo "Environment not confirmed as 'staging'. Intrusive tests will be DISABLED."
  ENVIRONMENT_CONFIRMED=NO
else
  ENVIRONMENT_CONFIRMED=YES
fi

# Resolve IP and check if private/local
RESOLVED_IP=$(resolve_target_ip "$TARGET" || true)
if [ -n "$RESOLVED_IP" ]; then
  if is_private_ip "$RESOLVED_IP"; then
    echo "Resolved target ${TARGET} -> ${RESOLVED_IP} (private/local)."
    IP_PRIVATE_OK=YES
  else
    echo "Resolved target ${TARGET} -> ${RESOLVED_IP} (public IP)."
    IP_PRIVATE_OK=NO
  fi
else
  echo "Could not resolve target IP. Proceeding but intrusive tests will be gated."
  IP_PRIVATE_OK=NO
fi

#################################
# Backup/snapshot verification
#################################

echo ""
echo "You must confirm a valid backup/snapshot exists that you can restore from."
echo "Acceptable confirmation examples:"
echo "  - a local backup file path that exists, OR"
echo "  - a cloud snapshot ID that you will be able to restore (we cannot validate cloud snapshots here)."

read -p "Enter local backup file path if present (or leave empty if you will provide a snapshot ID): " BACKUP_PATH
BACKUP_OK=NO
if [ -n "$BACKUP_PATH" ]; then
  if [ -f "$BACKUP_PATH" ]; then
    echo "Local backup file exists: ${BACKUP_PATH}"
    BACKUP_OK=YES
  else
    echo "Local backup file '${BACKUP_PATH}' not found. You may provide a snapshot ID instead, but we will still require typed confirmation."
    BACKUP_OK=NO
  fi
fi

if [ "$BACKUP_OK" != "YES" ]; then
  read -p "If you have a non-local snapshot ID or backup, type 'I_HAVE_A_SNAPSHOT' to acknowledge backups exist: " SNAP_ACK
  if [ "$SNAP_ACK" = "I_HAVE_A_SNAPSHOT" ]; then
    BACKUP_OK=YES
    echo "Snapshot acknowledgement recorded (we did not validate cloud snapshot presence)."
  else
    echo "Backup/snapshot not confirmed. Intrusive tests will be DISABLED."
    BACKUP_OK=NO
  fi
fi

#################################
# Final multi-gate acknowledgement for intrusive tests
#################################

echo ""
echo "INTRUSIVE TESTS: Additional safety gates."
echo "Intrusive testing can modify data, crash services, or impact availability."
echo "To enable intrusive tests you must satisfy ALL of the following:"
echo "  1) Environment confirmed as 'staging' (you typed 'staging')"
echo "  2) Target IP resolves to a private/local address OR you explicitly accept public IP risk"
echo "  3) Backup/snapshot confirmed (local file exists or snapshot ack provided)"
echo "  4) You type the exact acknowledgement phrase (case-sensitive):"
echo "       I_ACCEPT_RISK_AND_HAVE_STAGING_AND_BACKUP"
echo ""

ALLOW_INTRUSIVE=NO
read -p "Do you want to attempt to enable intrusive tests now? (type 'YES' to start gating checks): " TRY_INTRUSIVE
if [ "$TRY_INTRUSIVE" = "YES" ]; then
  if [ "$ENVIRONMENT_CONFIRMED" != "YES" ]; then
    echo "Environment not confirmed as staging. Cannot enable intrusive tests."
    ALLOW_INTRUSIVE=NO
  elif [ "$BACKUP_OK" != "YES" ]; then
    echo "Backup/snapshot not confirmed. Cannot enable intrusive tests."
    ALLOW_INTRUSIVE=NO
  elif [ "$IP_PRIVATE_OK" != "YES" ]; then
    # If IP not private, ask explicit acceptance
    echo "Target IP is not private/local. This is risky for intrusive testing."
    read -p "Type 'I_ACCEPT_PUBLIC_IP_RISK' to proceed despite public IP: " PUBLIC_ACK
    if [ "$PUBLIC_ACK" != "I_ACCEPT_PUBLIC_IP_RISK" ]; then
      echo "Public IP acceptance not provided. Intrusive tests disabled."
      ALLOW_INTRUSIVE=NO
    else
      echo "Public IP risk accepted by operator."
      ALLOW_INTRUSIVE=CANDIDATE
    fi
  else
    ALLOW_INTRUSIVE=CANDIDATE
  fi

  if [ "$ALLOW_INTRUSIVE" = "CANDIDATE" ]; then
    read -p "Type the exact acknowledgement to enable intrusive tests: I_ACCEPT_RISK_AND_HAVE_STAGING_AND_BACKUP: " FINAL_ACK
    if [ "$FINAL_ACK" = "I_ACCEPT_RISK_AND_HAVE_STAGING_AND_BACKUP" ]; then
      echo "Intrusive test gates satisfied. Intrusive tests will run."
      ALLOW_INTRUSIVE=YES
    else
      echo "Final acknowledgement incorrect. Intrusive tests disabled."
      ALLOW_INTRUSIVE=NO
    fi
  fi
else
  echo "Operator chose not to attempt intrusive gating. Intrusive tests disabled."
  ALLOW_INTRUSIVE=NO
fi

#################################
# Create results dir and run conservative scans (always run)
#################################

mkdir -p "${RESULTS_DIR}"
cd "${RESULTS_DIR}"

echo ""
echo "Starting conservative (non-intrusive) scans at $(date)..."
echo ""

# Test 1: Basic Port Check
echo "📌 Test 1/8: Basic Port Check..."
# Purpose:
#  - Confirm which expected ports are open. This is safe and non-destructive.
nmap -p ${PORTS} -T1 --open "${TARGET}" > 01-port-check.txt || true
echo "✅ Complete: 01-port-check.txt"
# Interpretation guidance:
# # - If expected ports are open: service reachable (attack surface exists).
# # - If closed: no network listener there; double-check config.

# Test 2: Service/Version Detection (conservative)
echo ""
echo "📌 Test 2/8: Service Version Detection..."
# Purpose:
#  - Identify banners in a conservative manner. No payloads/exploits.
nmap -p ${PORTS} -sV -T1 "${TARGET}" > 02-version-detection.txt || true
echo "✅ Complete: 02-version-detection.txt"
# Interpretation guidance:
# - Banner with specific version: indicates what software is present; outdated versions may warrant updates.
# - Masked/absent banners: reduces fingerprinting; still review for other signs of misconfiguration.

# Test 3: HTTP Methods
echo ""
echo "📌 Test 3/8: HTTP Methods (safe check)..."
nmap -p ${PORTS} --script http-methods -T1 "${TARGET}" > 03-http-methods.txt || true
echo "✅ Complete: 03-http-methods.txt"
# Interpretation guidance:
# - Only allow methods expected for the app (e.g., GET/POST).
# - Unexpected write methods (PUT/DELETE) should be audited and locked down.

# Test 4: HTTP Headers
echo ""
echo "📌 Test 4/8: HTTP Headers and Security-Related Headers..."
nmap -p ${PORTS} --script http-headers -T1 "${TARGET}" > 04-http-headers.txt || true
echo "✅ Complete: 04-http-headers.txt"
# Interpretation guidance:
# - Presence of CSP, HSTS, X-Frame-Options, X-Content-Type-Options, etc. is positive.
# - Verbose server headers may leak platform/version info; consider minimizing.

# Test 5: CORS Configuration
echo ""
echo "📌 Test 5/8: CORS Configuration (safe check)..."
nmap -p ${PORTS} --script http-cors -T1 "${TARGET}" > 05-cors-check.txt || true
echo "✅ Complete: 05-cors-check.txt"
# Interpretation guidance:
# - Wildcard CORS on APIs exposing sensitive data is risky; restrict origins where possible.

# Test 6: HTTP Title and Server Summary
echo ""
echo "📌 Test 6/8: HTTP Title & Server Header Summary..."
nmap -p ${PORTS} --script http-title,http-server-header -T1 "${TARGET}" > 06-http-summary.txt || true
echo "✅ Complete: 06-http-summary.txt"
# Interpretation guidance:
# - Confirms you targeted the expected app pages; unexpected titles may indicate incorrect target.

#################################
# Intrusive tests (only if all gates satisfied)
#################################
if [ "${ALLOW_INTRUSIVE}" = "YES" ]; then
  echo ""
  echo ">>> Running INTRUSIVE tests (staging/test only)."
  echo "Note: These scripts may modify data, trigger WAF/IDS, or disrupt service."
  echo ""

  # Test 7: Vulnerability script (intrusive)
  echo "📌 Test 7/8: Vulnerability Scan (intrusive - conservative)"
  # Purpose:
  #  - Probe for known vulnerabilities using NSE vuln family. This can be intrusive.
  # What a SAFE result looks like:
  #  - No vulnerable matches found -> lower immediate risk but continue hardening.
  # What to do if findings are returned:
  #  - Triage each finding, verify whether it is relevant to your exact version/config,
  #    and plan remediation/testing in staging.
  nmap -p ${PORTS} --script vuln -T2 "${TARGET}" > 07-vulnerability-scan.txt || echo "vuln script completed with errors"
  echo "✅ Complete: 07-vulnerability-scan.txt"

  # Test 8: SQL injection and XSS tests (intrusive, payload-based)
  echo ""
  echo "📌 Test 8/8: Injection & XSS checks (intrusive, payload-based)"
  # Purpose:
  #  - Check for SQLi/XSS vectors by sending test payloads. These may create log entries or test data.
  # What a SAFE result looks like:
  #  - No injection vectors triggered; application handles inputs safely.
  # What to do if results are concerning:
  #  - Reproduce with safe, instrumented test requests, review sanitization/ORM usage, and add input validation + parameterized queries.
  nmap -p ${PORTS} --script "http-sql-injection,http-xss*" -T2 "${TARGET}" > 08-injection-xss.txt || echo "injection/xss scripts completed with errors"
  echo "✅ Complete: 08-injection-xss.txt"

  echo ""
  echo "Intrusive tests finished. Review the output files carefully and act on confirmed findings."
else
  echo ""
  echo "Intrusive tests are DISABLED -- you did not satisfy all gating checks."
  echo "If you want to enable intrusive tests later, re-run the script and satisfy the gating requirements."
fi

#################################
# Final summary & safe next steps
#################################
echo ""
echo "All requested scans completed at $(date). Results directory:"
echo "  ${PWD}"
echo ""
echo "Recommended next steps (non-exhaustive):"
echo "  - Review each output file in ${RESULTS_DIR} and triage findings."
echo "  - For any confirmed vulnerabilities, patch/update the affected component in staging first."
echo "  - Re-run targeted tests after fixes to confirm remediation."
echo "  - Consider running authenticated scans (credentialed) in staging for higher coverage."
echo "  - For production resilience testing (DoS, chaos), schedule with ops and perform under a maintenance window."
echo ""
echo "Important: This script intentionally avoids giving exploit details. If you find serious issues,"
echo "engage a professional pentester or security team to verify and remediate safely."
echo ""
echo "Generated files:"
ls -lh *.txt || true

exit 0