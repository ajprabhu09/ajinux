int a = 1;

int caller() {
    return 1;
}

int main() {
    a += 1;
    return caller();
}