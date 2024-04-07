const loginSection = document.getElementById("login-section");
const twoFASection = document.getElementById("2fa-section");
const signupSection = document.getElementById("signup-section");

const signupLink = document.getElementById("signup-link");
const twoFALoginLink = document.getElementById("2fa-login-link");
const signupLoginLink = document.getElementById("signup-login-link");

signupLink.addEventListener("click", (e) => {
    e.preventDefault();

    loginSection.style.display = "none";
    twoFASection.style.display = "none";
    signupSection.style.display = "block";
});

twoFALoginLink.addEventListener("click", (e) => {
    e.preventDefault();

    loginSection.style.display = "block";
    twoFASection.style.display = "none";
    signupSection.style.display = "none";
});

signupLoginLink.addEventListener("click", (e) => {
    e.preventDefault();

    loginSection.style.display = "block";
    twoFASection.style.display = "none";
    signupSection.style.display = "none";
});

// -----------------------------------------------------

const loginForm = document.getElementById("login-form");
const loginButton = document.getElementById("login-form-submit");
const loginErrAlter = document.getElementById("login-err-alert");

loginButton.addEventListener("click", (e) => {
    e.preventDefault();

    const email = loginForm.email.value;
    const password = loginForm.password.value;

    fetch('/login', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({ email, password }),
    }).then(response => {
        if (response.status === 206) {
            TwoFAForm.email.value = email;
            response.json().then(data => {
                TwoFAForm.login_attempt_id.value = data.loginAttemptId;
            });

            loginForm.email.value = "";
            loginForm.password.value = "";

            loginSection.style.display = "none";
            twoFASection.style.display = "block";
            signupSection.style.display = "none";
            loginErrAlter.style.display = "none";
        } else if (response.status === 200) {
            loginForm.email.value = "";
            loginForm.password.value = "";
            loginErrAlter.style.display = "none";
            alert("You have successfully logged in.");
        } else {
            response.json().then(data => {
                let error_msg = data.error;
                if (error_msg !== undefined && error_msg !== null && error_msg !== "") {
                    loginErrAlter.innerHTML = `<span><strong>Error: </strong>${error_msg}</span>`;
                    loginErrAlter.style.display = "block";
                } else {
                    loginErrAlter.style.display = "none";
                }
            });
        }
    });
});

const signupForm = document.getElementById("signup-form");
const signupButton = document.getElementById("signup-form-submit");
const signupErrAlter = document.getElementById("signup-err-alert");

signupButton.addEventListener("click", (e) => {
    e.preventDefault();

    const email = signupForm.email.value;
    const password = signupForm.password.value;
    const requires2FA = signupForm.twoFA.checked;

    fetch('/signup', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({ email, password, requires2FA }),
    }).then(response => {
        if (response.ok) {
            signupForm.email.value = "";
            signupForm.password.value = "";
            signupForm.twoFA.checked = false;
            signupErrAlter.style.display = "none";
            alert("You have successfully created a user.");
            loginSection.style.display = "block";
            twoFASection.style.display = "none";
            signupSection.style.display = "none";
        } else {
            response.json().then(data => {
                let error_msg = data.error;
                if (error_msg !== undefined && error_msg !== null && error_msg !== "") {
                    signupErrAlter.innerHTML = `<span><strong>Error: </strong>${error_msg}</span>`;
                    signupErrAlter.style.display = "block";
                } else {
                    signupErrAlter.style.display = "none";
                }
            });
        }
    });
});

const TwoFAForm = document.getElementById("2fa-form");
const TwoFAButton = document.getElementById("2fa-form-submit");
const TwoFAErrAlter = document.getElementById("2fa-err-alert");

TwoFAButton.addEventListener("click", (e) => {
    e.preventDefault();

    const email = TwoFAForm.email.value;
    const loginAttemptId = TwoFAForm.login_attempt_id.value;
    const TwoFACode = TwoFAForm.email_code.value;

    fetch('/verify-2fa', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({ email, loginAttemptId, "2FACode": TwoFACode }),
    }).then(response => {
        if (response.ok) {
            TwoFAForm.email.value = "";
            TwoFAForm.email_code.value = "";
            TwoFAForm.login_attempt_id.value = "";
            TwoFAErrAlter.style.display = "none";
            alert("You have successfully logged in.");
            loginSection.style.display = "block";
            twoFASection.style.display = "none";
            signupSection.style.display = "none";
        } else {
            response.json().then(data => {
                let error_msg = data.error;
                if (error_msg !== undefined && error_msg !== null && error_msg !== "") {
                    TwoFAErrAlter.innerHTML = `<span><strong>Error: </strong>${error_msg}</span>`;
                    TwoFAErrAlter.style.display = "block";
                } else {
                    TwoFAErrAlter.style.display = "none";
                }
            });
        }
    });
});