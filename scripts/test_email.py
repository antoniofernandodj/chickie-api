# import smtplib
# import ssl
# from email.mime.text import MIMEText
# from email.mime.multipart import MIMEMultipart


# def load_env(path=".env"):
#     env = {}
#     with open(path) as f:
#         for line in f:
#             line = line.strip()
#             if not line or line.startswith("#") or "=" not in line:
#                 continue
#             key, _, value = line.partition("=")
#             env[key.strip()] = value.strip().strip('"').strip("'")
#     return env


# env = load_env()

# name   = env["SMTP_NAME"]
# server = env["SMTP_SERVER"]
# port   = int(env["SMTP_PORT"])
# user   = env["SMTP_USER"]
# password = env["SMTP_PASS"]

# to = "antoniofernandodj@outlook.com"

# msg = MIMEMultipart("alternative")
# msg["Subject"] = "Teste de envio — Chickie"
# msg["From"]    = f"{name} <{user}>"
# msg["To"]      = to

# text = "Se você está lendo isto, o envio de email está funcionando."
# html = """\
# <html>
#   <body>
#     <p>Se você está lendo isto, o envio de email está <strong>funcionando</strong>. ✅</p>
#   </body>
# </html>
# """

# msg.attach(MIMEText(text, "plain"))
# msg.attach(MIMEText(html, "html"))

# context = ssl.create_default_context()

# with smtplib.SMTP(server, port) as smtp:
#     smtp.ehlo()
#     smtp.starttls(context=context)
#     smtp.login(user, password)
#     smtp.sendmail(user, to, msg.as_string())

# print(f"Email enviado para {to}")
