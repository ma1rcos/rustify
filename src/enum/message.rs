pub enum Message {
    // Global
    UnknowError,
    NotAuthorized,
    // User
    EmailAlreadyUsed,
    // TelephoneAlreadyUsed,
    UserCreatedSuccessfully,
    // UserUpdatedSuccessfully,
    // UserDeletedSuccessfully,
    _RequestDeletionMadeSuccessfully,
    UserNotFound,
    UserAuthenticatedSuccessfully,
    InvalidCredentials,
    // Request
    _RequestCreatedSuccesfully,
    _RequestDeletedSuccesfully,
    // Category
    // CategoryAlreadyExists,
    _SomeOfTheCategoriesProvidedDoNotExist,
    // CategoryNotFound,
    // CategoryCreatedSuccesfully,
    // CategoryUpdatedSuccesfully,
    // CategoryDeletedSuccesfully,
    // CategoriesFoundSuccesfully
}

impl Message {
    pub fn get_message(&self) -> String {
        match self {
            Message::UnknowError => String::from("Erro desconhecido. Se o problema persistir, contate o suporte."),
            Message::NotAuthorized => String::from("Não autorizado."),
            Message::EmailAlreadyUsed => String::from("Este e-mail já está em uso. Tente outro."),
            // Message::TelephoneAlreadyUsed => String::from("Este número de telefone já está em uso. Tente outro."),
            Message::UserCreatedSuccessfully => String::from("Usuário criado com sucesso!"),
            // Message::UserUpdatedSuccessfully => String::from("Usuário atualizado com sucesso!"),
            // Message::UserDeletedSuccessfully => String::from("Usuário excluído com sucesso!"),
            Message::_RequestDeletionMadeSuccessfully => String::from("Solicitação de exclusão feita com sucesso! Você tem 120 horas para mudar de ideia."),
            Message::UserNotFound => String::from("Usuário não encontrado."),
            Message::UserAuthenticatedSuccessfully => String::from("Usuário autenticado com sucesso!"),
            Message::InvalidCredentials => String::from("Credenciais inválidas. Verifique e-mail e senha, e tente novamente."),
            Message::_RequestCreatedSuccesfully => String::from("Solicitação criada com sucesso!"),
            Message::_RequestDeletedSuccesfully => String::from("Solicitação excluída com sucesso!"),
            // Message::CategoryAlreadyExists => String::from("Esta categoria já existe. Tente outra."),
            // Message::CategoriesFoundSuccesfully => String::from("Categorias encontradas com sucesso!"),
            Message::_SomeOfTheCategoriesProvidedDoNotExist => String::from("Alguma das categorias informadas não existe."),
            // Message::CategoryNotFound => String::from("Categoria não encontrada."),
            // Message::CategoryCreatedSuccesfully => String::from("Categoria criada com sucesso!"),
            // Message::CategoryUpdatedSuccesfully => String::from("Categoria atualizada com sucesso!"),
            // Message::CategoryDeletedSuccesfully => String::from("Categoria excluída com sucesso!"),
        }
    }
}