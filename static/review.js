const approveButton = document.getElementById("approve");
const rejectButton = document.getElementById("reject");
const video = document.getElementById("video");
const comment = document.getElementById("comment");
const approveLoader = document.getElementById("approve-loader");
const rejectLoader = document.getElementById("reject-loader");

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
				call_sid: video.dataset.callSid,
				comment: comment.value,
			}),
		});
	} catch (error) {
		console.error(error);
	}
	loader.style.display = "none";
	window.location.reload();
}

