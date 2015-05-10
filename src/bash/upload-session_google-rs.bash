#!/bin/bash

video_path="${1:?Specify the file to upload}"
issue=${2:?The id of the issue to post the video to, like 124}
repo=${3-google-apis-rs}
thumbnail="${video_path%.*}.jpg"
video_basename="${video_path##*/}"
title="${video_basename% - 1080p-web-HQ.*}"

if [[ ! -e $thumbnail ]]; then
	echo "thumbnail file at '$thumbnail' doesn't exist"
	exit 2
fi

# my channel
channel_id=UCkn47As_FHFviIqEVAXx1vw
# Let's build a youtube uploader
playlist_id=PLMHbQxe1e9Mnnqj3Hs1hRDUXFEK-TgCnz

# VIDEO UPLOAD
echo "Uploading video: '${title} ..."
video_id=$(youtube3 videos insert \
			-r snippet \
				title="${title}" \
				description="https://github.com/Byron/${repo}/issues/${issue}" \
				tags="Google APIs" \
				tags=Google \
				tags=rust-lang \
				tags=diary \
				tags=OSS \
				category-id=28 \
			..status \
				privacy-status=public \
				embeddable=true \
				license=youtube \
			-p notify-subscribers=false \
			-u resumable "${video_path}" | tee last-${repo}-upload.json | jq -r .id)

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
echo "Adding video '${video_id}' to GitHub issue ${issue} ..."
gh is --repo ${repo} \
 		-c $"You can watch the development stream [on youtube](https://youtu.be/${video_id}).

[\![${title}](${thumbnail_url})](https://youtu.be/${video_id})" \
 		${issue}

echo "DONE"