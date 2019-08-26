#!/bin/bash
input="./test_urls_for_ent.csv"
invalid_output="./invalid.csv"
# rm $invalid_output
# touch $invalid_output
total=$(awk 'END {print NR}' $input)
num_valid=0
while IFS= read -r url
do
  domain=$(echo "$url" | awk -F/ '{print $3}')
  path=$(echo "$url" | awk -F$domain '{print $2}')
  cert="https://$domain/cdn-fpw/sxg/cert.pem.msg"
  valid=$(curl -s -H "Accept:application/signed-exchange;v=b3" -H "AMP-Cache-Transform:google;v=1..2" $url | dump-signedexchange -verify -payload=false -cert <(curl -s -S $cert) -json | jq .Valid)
  if [ "${valid:false}" = "true" ] ; then
    echo "$url: true"
    num_valid=$((num_valid + 1))
  else
    cert="https://www.$domain/cdn-fpw/sxg/cert.pem.msg"
    valid=$(curl -s -H "Accept:application/signed-exchange;v=b3" -H "AMP-Cache-Transform:google;v=1..2" https://www.$domain$path | dump-signedexchange -verify -payload=false -cert <(curl -s -S $cert) -json | jq .Valid)
      if [ "${valid:false}" = "true" ] ; then
        echo "$url: true"
        num_valid=$((num_valid + 1))
      else
        echo "$url: false"
        echo $url >> $invalid_output
      fi
  fi
done < "$input"
echo "$num_valid/$total valid sxg"
