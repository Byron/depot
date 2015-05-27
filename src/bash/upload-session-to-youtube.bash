#!/bin/bash

video_path="${1:?Specify the file to upload}"
issue=${2:?The id of the issue to post the video to, like 124}
config=${3:?The name of the configuration file to load, like google-rs}

config_file=${0%/*}/./${config}.sh
if [[ ! -f "${config_file}" ]]; then
	echo "Didn't find configuration file at '${config_file}'"
	exit 2
fi

source ${config_file}

if [[ -z ${playlist_id} ]]; then
	echo "playlist_id must be set in configuration"
	exit 2
fi

if [[ -z ${default_repo} ]]; then
	echo "default_repo must be set in configuration"
	exit 2
fi

if [[ -z ${github_user} ]]; then
	echo "github_user must be set in configuration"
	exit 2
fi

if [[ -z ${privacy} ]]; then
	echo "privacy must be set in configuration"
	exit 2
fi

repo=${4-${default_repo}}

thumbnail="${video_path%.*}.jpg"
video_basename="${video_path##*/}"
title="${video_basename% - 1080p-web-HQ.*}"

if [[ ! -e $thumbnail ]]; then
	echo "thumbnail file at '$thumbnail' doesn't exist"
	exit 2
fi

# my channel
channel_id=UCkn47As_FHFviIqEVAXx1vw

# VIDEO UPLOAD
echo "Uploading video: '${title} ..."
video_id=$(youtube3 videos insert \
			-r snippet \
				title="${title}" \
				description="https://github.com/${github_user}/${repo}/issues/${issue}" \
				tags="Google APIs" \
				tags=Google \
				tags=rust-lang \
				tags=diary \
				tags=OSS \
				category-id=28 \
			..status \
				privacy-status=${privacy} \
				embeddable=true \
				license=youtube \
			-p notify-subscribers=false \
			-u simple "${video_path}" | tee last-${repo}-upload.json | jq -r .id)

if [[ -z ${video_id} ]]; then
	echo "Upload failed !"
	exit 3
fi

# THUMBNAIL
echo "Setting thumbnail for '${video_id}' ... "
thumbnail_url=$(youtube3 thumbnails set $video_id \
			    -u simple "${thumbnail}" | jq -r ".items[0].medium.url")

if [[ -z ${thumbnail_url} ]]; then
	echo "Thumbnail upload failed !"
	exit 3
fi


# PLAYLIST SETUP
echo "Adding '${video_id}' to playlist"
youtube3 playlist-items insert \
		-r snippet \
			channel-id=${channel_id} \
			playlist-id=${playlist_id} \
			resource-id \
				video-id=${video_id} \
				kind="youtube#video" \
				channel-id=${channel_id} >/dev/null

# Add issue comment
if [[ ${repo} != NONE ]]; then
	echo "Adding video '${video_id}' to GitHub issue ${issue} ..."
	gh is --user "${github_user}" --repo "${repo}" \
 		-c $"You can watch the development stream [on youtube](https://youtu.be/${video_id}).

*\`${title}\`*

[\![thumb](${thumbnail_url})](https://youtu.be/${video_id})" \
 		${issue}

fi

echo "DONE"

read -p "Do you want want to delete the 2 source files ? y/n [n]: " DELETE_SOURCES
if [[ "${DELETE_SOURCES}" = y ]]; then
	rm -v "${video_path}"
	rm -v "${thumbnail}"
fi