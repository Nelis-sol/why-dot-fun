const approveButton = document.getElementById("approve");
const rejectButton = document.getElementById("reject");
const video = document.getElementById("video");
const comment = document.getElementById("comment");
const approveLoader = document.getElementById("approve-loader");
const rejectLoader = document.getElementById("reject-loader");

var call_sid = null;

approveButton.addEventListener("click", async () => {
	await processDraft("approve", approveLoader);
});

rejectButton.addEventListener("click", async () => {
	await processDraft("reject", rejectLoader);
});

async function processDraft(action, loader) {
	loader.style.display = "block";
	try {
		await fetch(`/review/${action}`, {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify({
				call_sid,
				comment: comment.value,
			}),
		});
	} catch (error) {
		console.error(error);
	}
	loader.style.display = "none";
	await fetchNextDraft();
}

async function fetchNextDraft() {
	try {
		const response = await fetch("/review/next", {
			method: "GET",
		});

		if (response.ok) {
			const data = await response.json();
			call_sid = data.call_sid;
			video.src = `/review/drafts/${call_sid}.mp4`;
			comment.value = data.comment;
			approveButton.disabled = false;
			rejectButton.disabled = false;
		} else {
			call_sid = null;
			video.src = "";
			approveButton.disabled = true;
			rejectButton.disabled = true;
		}
	} catch (error) {
		console.error(error);
	}
}

fetchNextDraft();
