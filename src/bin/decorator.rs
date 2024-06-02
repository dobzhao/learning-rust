trait TProductRepository<Command> {
    fn execute(&self, cmd: Command);
}
struct SqlRepository {
    connection: String,
}
enum ProductOperation {
    InsertProduct(Product),
    DeleteProduct(i32),
    UpdateProduct(Product),
}

impl TProductRepository<ProductOperation> for SqlRepository {
    fn execute(&self, cmd: ProductOperation) {
        if let ProductOperation::InsertProduct(product) = cmd {
            println!(
                "insert product! id:{}, name: {}, price: {}, connection: {}",
                product.id, product.name, product.price, self.connection
            )
        } else if let ProductOperation::DeleteProduct(id) = cmd {
            println!(
                "delete product! id: {}, connection: {}",
                id, self.connection
            )
        } else if let ProductOperation::UpdateProduct(product) = cmd {
            println!(
                "update product! id:{}, name: {}, price: {}, connection: {}",
                product.id, product.name, product.price, self.connection
            );
        }
    }
}
struct Product {
    id: i32,
    name: String,
    price: f64,
}

struct ProductRepositoryAuditDecorator<T> {
    decoratee: T,
}
impl<T> ProductRepositoryAuditDecorator<T> {
    fn audit(&self) {
        println!("auditting...")
    }
}
impl<T: TProductRepository<C>, C> TProductRepository<C> for ProductRepositoryAuditDecorator<T> {
    fn execute(&self, cmd: C) {
        self.audit();
        self.decoratee.execute(cmd);
    }
}

fn main() {
    let sql_repository = SqlRepository {
        connection: "mysql://root:123456@localhost/test".to_string(),
    };
    let insert_product = ProductOperation::InsertProduct(Product {
        id: 100,
        name: "金牌".to_string(),
        price: 123456789.0,
    });
    // sql_repository.execute(insert_product);

    let decorator = ProductRepositoryAuditDecorator {
        decoratee: sql_repository,
    };

    decorator.execute(insert_product);

    let update_product = ProductOperation::UpdateProduct(Product {
        id: 100,
        name: "银牌".to_string(),
        price: 6666666.0,
    });
    decorator.execute(update_product);

    let delete_product = ProductOperation::DeleteProduct(100);
    decorator.execute(delete_product);
}
