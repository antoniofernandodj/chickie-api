# Integração Twillio

A especificação a seguir visa fornecer as informações necessárias para a integração do WhatsApp com o sistema de vendas da empresa. É destinado a tratar as requisições de vendas como uma interface de usuario em chat para a venda de produtos e
também como um sistema de notificação de atualização de status de pedidos.

## Provedor

O provedor para o sistema de whatsapp é a Twilio.
seu webhook foi configurado para enviar as mensagens do whatsapp para /webhook/whatsapp
e a confirmação to status de entrega da mensagem será enviado para /webhook/whatsapp/delivery-satatus

## API

### Exemplo de envio de mensagem iniciada por empresa

to_whatsapp="5521992784394"
from_whatsapp="14155238886"
content_id="HXb5b62575e6e4ff6129ad7c8efe1f983e"
path="https://api.twilio.com/2010-04-01/Accounts/${acount_sid}/Messages.json"
curl "${path}" -X POST \
    --data-urlencode "To=whatsapp:+{to_whatsapp}" \
    --data-urlencode "From=whatsapp:+{from_whatsapp}" \
    --data-urlencode "ContentSid={content_id}" \
    --data-urlencode 'ContentVariables={"1":"12/1","2":"3pm"}' \
    -u "${acount_sid}:${auth_token}"

### Exemplo de mensagem iniciada por usuario

text="Your appointment is coming up on July 21 at 3PM"
from_whatsapp="14155238886"
to_whatsapp="5521992784394"
path="https://api.twilio.com/2010-04-01/Accounts/${acount_sid}/Messages.json"
curl "{path}" -X POST \
--data-urlencode 'To=whatsapp:+{to_whatsapp}' \
--data-urlencode 'From=whatsapp:+{from_whatsapp}' \
--data-urlencode "Body={text}" \
-u "${acount_sid}:${auth_token}"
